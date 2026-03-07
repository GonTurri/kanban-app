import React from 'react';
import { Droppable, Draggable } from '@hello-pangea/dnd';
import type { Column, Item, ColumnType } from '../../types';
import { ItemCard } from './ItemCard';

interface ColumnViewProps {
    column: Column;
    index: number;
    canEdit: boolean;
    onOpenCreateItem: (columnId: string, columnName: string) => void;
    onItemClick: (item: Item) => void;
    onLoadMore: (columnId: string, offset: number) => void;
    onEditColumn: (column: Column) => void;
    onDeleteColumn: (columnId: string) => Promise<void>;
}

export const ColumnView: React.FC<ColumnViewProps> = ({
                                                          column, index, canEdit, onOpenCreateItem, onItemClick, onLoadMore, onEditColumn, onDeleteColumn
                                                      }) => {

    const handleDeleteCol = () => {
        if (!canEdit) return;
        if (window.confirm(`Delete column "${column.name}" and ALL its tasks?`)) {
            onDeleteColumn(column.id);
        }
    };

    const getLimit = (kind: ColumnType): number | null => {
        if (kind.type === 'wip' || kind.type === 'todo') {
            return kind.limit;
        }
        return null;
    };

    const limit = getLimit(column.kind);
    const isOverLimit = limit !== null && column.items.length > limit;

    return (
        <Draggable draggableId={column.id} index={index} isDragDisabled={!canEdit}>
            {(provided) => (
                <div
                    ref={provided.innerRef}
                    {...provided.draggableProps}
                    className="flex flex-col bg-gray-100/80 rounded-xl w-80 shrink-0 border border-gray-200 max-h-full h-full"
                >
                    <div {...provided.dragHandleProps} className="p-4 shrink-0 flex justify-between items-center group border-b border-gray-200/50">
                        <h3
                            className={`font-semibold text-gray-700 truncate pr-2 ${canEdit ? 'cursor-pointer hover:text-blue-600' : ''}`}
                            onDoubleClick={() => canEdit && onEditColumn(column)}
                            title={canEdit ? "Double click to rename settings" : ""}
                        >
                            {column.name}
                        </h3>
                        <div className="flex items-center gap-2">
                            <span className={`text-xs font-bold px-2 py-0.5 rounded-full ${isOverLimit ? 'bg-red-100 text-red-700' : 'bg-gray-200 text-gray-600'}`}>
                                {column.items.length} {limit ? `/ ${limit}` : ''}
                            </span>
                            {canEdit && (
                                <div className="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                                    <button onClick={() => onEditColumn(column)} className="text-gray-400 hover:text-blue-600 p-1" title="Edit Column">✏️</button>
                                    <button onClick={handleDeleteCol} className="text-gray-400 hover:text-red-600 p-1" title="Delete Column">🗑️</button>
                                </div>
                            )}
                        </div>
                    </div>

                    <Droppable droppableId={column.id} type="item">
                        {(dropProvided, dropSnapshot) => (
                            <div
                                ref={dropProvided.innerRef}
                                {...dropProvided.droppableProps}
                                className={`flex-1 px-3 pt-3 overflow-y-auto transition-colors ${
                                    dropSnapshot.isDraggingOver ? 'bg-blue-50/50' : ''
                                }`}
                            >
                                {column.items.map((item, idx) => (
                                    <ItemCard key={item.id} item={item} index={idx} canEdit={canEdit} onClick={onItemClick} />
                                ))}
                                {dropProvided.placeholder}

                                {column.items.length >= 10 && column.items.length % 10 === 0 && (
                                    <button
                                        onClick={() => onLoadMore(column.id, column.items.length)}
                                        className="w-full py-2 mb-3 text-xs font-semibold text-blue-600 bg-blue-50 hover:bg-blue-100 rounded-lg transition-colors shadow-sm"
                                    >
                                        Load More ↓
                                    </button>
                                )}
                            </div>
                        )}
                    </Droppable>

                    {canEdit && (
                        <div className="p-3 shrink-0 bg-gray-100/80 rounded-b-xl">
                            <button onClick={() => onOpenCreateItem(column.id, column.name)} className="flex items-center text-sm font-medium text-gray-500 hover:text-gray-900 hover:bg-gray-200 w-full p-2 rounded-lg transition-colors">
                                <span className="mr-2 text-lg leading-none">+</span> Add a card
                            </button>
                        </div>
                    )}
                </div>
            )}
        </Draggable>
    );
};