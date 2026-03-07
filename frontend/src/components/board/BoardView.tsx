import React, { useState } from 'react';
import { DragDropContext, Droppable, type DropResult } from '@hello-pangea/dnd';
import type { Board, Item, ItemPriority, BoardRole, ColumnType, Column } from '../../types';
import { ColumnView } from './ColumnView';
import { ItemModal } from './ItemModal';
import { BoardSettingsModal } from './BoardSettingsModal';
import { ColumnModal } from './ColumnModal';
import { CreateItemModal } from './CreateItemModal';
import { useAuth } from '../../context/AuthContext';

interface BoardViewProps {
    board: Board;
    onMoveItem: (itemId: string, sourceColId: string, destColId: string, sourceIdx: number, destIdx: number) => void;
    onAddItem: (columnId: string, title: string, desc: string | undefined, prio: ItemPriority, assignedTo: string | null) => Promise<void>;
    onUpdateItem: (itemId: string, title: string, desc: string | undefined, prio: ItemPriority, assignedTo: string | null) => Promise<void>;
    onDeleteItem: (itemId: string) => Promise<void>;
    onLoadMoreItems: (columnId: string, offset: number) => void;
    onAddMember: (email: string, role: BoardRole) => Promise<void>;
    onUpdateMemberRole: (userId: string, role: BoardRole) => Promise<void>;
    onRemoveMember: (userId: string) => Promise<void>;
    onAddColumn: (name: string, kind: ColumnType, targetIndex: number) => Promise<void>;
    onUpdateColumn: (columnId: string, name: string, kind: ColumnType) => Promise<void>;
    onDeleteColumn: (columnId: string) => Promise<void>;
    onMoveColumn: (columnId: string, targetIndex: number) => Promise<void>;
}

export const BoardView: React.FC<BoardViewProps> = ({
                                                        board, onMoveItem, onAddItem, onUpdateItem, onDeleteItem, onLoadMoreItems,
                                                        onAddMember, onUpdateMemberRole, onRemoveMember,
                                                        onAddColumn, onUpdateColumn, onDeleteColumn, onMoveColumn
                                                    }) => {
    const { user } = useAuth();

    // UI States
    const [selectedItem, setSelectedItem] = useState<Item | null>(null);
    const [isSettingsOpen, setIsSettingsOpen] = useState(false);

    // Modal States
    const [columnModal, setColumnModal] = useState<{ isOpen: boolean; column?: Column; targetIndex?: number }>({ isOpen: false });
    const [createItemTarget, setCreateItemTarget] = useState<{ id: string; name: string } | null>(null);

    const isOwner = user?.id === board.owner_id;
    const userRole = isOwner ? 'owner' : board.members.find(m => m.user_id === user?.id)?.role || 'viewer';

    const canEdit = userRole === 'owner' || userRole === 'editor';
    const canManageMembers = userRole === 'owner';

    const handleDragEnd = (result: DropResult) => {
        const { destination, source, draggableId, type } = result;
        if (!destination || (destination.droppableId === source.droppableId && destination.index === source.index)) return;

        if (type === 'column') {
            onMoveColumn(draggableId, destination.index);
        } else {
            onMoveItem(draggableId, source.droppableId, destination.droppableId, source.index, destination.index);
        }
    };

    return (
        <div className="flex flex-col h-full">
            <div className="px-8 pt-6 pb-2 flex justify-between items-center">
                <h2 className="text-xl font-bold text-gray-800">{board.title}</h2>
                <button onClick={() => setIsSettingsOpen(true)} className="px-4 py-2 text-sm font-medium bg-white border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 shadow-sm">
                    ⚙️ Board Settings
                </button>
            </div>

            <DragDropContext onDragEnd={handleDragEnd}>
                <Droppable droppableId="board" direction="horizontal" type="column">
                    {(provided) => (
                        <div ref={provided.innerRef} {...provided.droppableProps} className="flex gap-6 overflow-x-auto px-8 pb-8 pt-4 h-full items-start">
                            {board.columns.map((column, index) => (
                                <ColumnView
                                    key={column.id}
                                    column={column}
                                    index={index}
                                    canEdit={canEdit}
                                    onOpenCreateItem={(id, name) => setCreateItemTarget({ id, name })}
                                    onItemClick={setSelectedItem}
                                    onLoadMore={onLoadMoreItems}
                                    onEditColumn={(col) => setColumnModal({ isOpen: true, column: col })}
                                    onDeleteColumn={onDeleteColumn}
                                />
                            ))}
                            {provided.placeholder}

                            {canEdit && (
                                <button
                                    onClick={() => setColumnModal({ isOpen: true, targetIndex: board.columns.length })}
                                    className="shrink-0 w-80 h-14 rounded-xl border-2 border-dashed border-gray-300 flex items-center justify-center text-gray-500 font-medium hover:bg-gray-50 hover:text-blue-600 transition-colors"
                                >
                                    + Add New Column
                                </button>
                            )}
                        </div>
                    )}
                </Droppable>
            </DragDropContext>

            {/* Modals */}
            {selectedItem && (
                <ItemModal
                    item={selectedItem}
                    columns={board.columns}
                    members={board.members}
                    canEdit={canEdit}
                    onClose={() => setSelectedItem(null)}
                    onUpdate={onUpdateItem}
                    onDelete={onDeleteItem}
                />
            )}

            {createItemTarget && (
                <CreateItemModal
                    columnName={createItemTarget.name}
                    members={board.members}
                    onClose={() => setCreateItemTarget(null)}
                    onSave={(t, d, p, a) => onAddItem(createItemTarget.id, t, d, p, a)}
                />
            )}

            {columnModal.isOpen && (
                <ColumnModal
                    column={columnModal.column}
                    onClose={() => setColumnModal({ isOpen: false })}
                    onSave={async (name, kind) => {
                        if (columnModal.column) {
                            await onUpdateColumn(columnModal.column.id, name, kind);
                        } else {
                            await onAddColumn(name, kind, columnModal.targetIndex || 0);
                        }
                    }}
                />
            )}

            {isSettingsOpen && (
                <BoardSettingsModal
                    board={board}
                    canManage={canManageMembers}
                    onClose={() => setIsSettingsOpen(false)}
                    onAddMember={onAddMember}
                    onUpdateRole={onUpdateMemberRole}
                    onRemoveMember={onRemoveMember}
                />
            )}
        </div>
    );
};