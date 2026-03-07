import React from 'react';
import { Draggable } from '@hello-pangea/dnd';
import type { Item } from '../../types';

interface ItemCardProps {
    item: Item;
    index: number;
    canEdit: boolean;
    onClick: (item: Item) => void;
}

export const ItemCard: React.FC<ItemCardProps> = ({ item, index, canEdit, onClick }) => {
    const priorityColors = {
        low: 'bg-blue-100 text-blue-800',
        medium: 'bg-yellow-100 text-yellow-800',
        high: 'bg-red-100 text-red-800',
    };

    return (
        <Draggable draggableId={item.id} index={index} isDragDisabled={!canEdit}>
            {(provided, snapshot) => (
                <div
                    ref={provided.innerRef}
                    {...provided.draggableProps}
                    {...provided.dragHandleProps}
                    onClick={() => onClick(item)}
                    className={`p-4 mb-3 bg-white rounded-lg shadow-sm border transition-shadow cursor-pointer ${
                        snapshot.isDragging ? 'border-blue-500 shadow-lg' : 'border-gray-200 hover:border-gray-300'
                    } ${!canEdit ? 'opacity-90 cursor-default' : ''}`}
                >
                    <div className="flex justify-between items-start mb-2">
                        <h4 className="font-medium text-gray-900 text-sm">{item.title}</h4>
                        <span className={`text-[10px] px-2 py-0.5 rounded-full font-bold uppercase tracking-wider ${priorityColors[item.priority]}`}>
                           {item.priority}
                        </span>
                    </div>
                    {item.description && (
                        <p className="text-xs text-gray-500 line-clamp-2">{item.description}</p>
                    )}
                    {item.assigned_to && (
                        <div className="mt-3 flex items-center justify-end">
                            <span className="w-6 h-6 rounded-full bg-blue-100 text-blue-700 flex items-center justify-center text-xs font-bold" title="Assigned">
                                👤
                            </span>
                        </div>
                    )}
                </div>
            )}
        </Draggable>
    );
};