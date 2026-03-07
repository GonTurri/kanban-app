import React, { useState } from 'react';
import type { Column, ColumnType } from '../../types';

interface ColumnModalProps {
    column?: Column;
    onClose: () => void;
    onSave: (name: string, kind: ColumnType) => Promise<void>;
}

export const ColumnModal: React.FC<ColumnModalProps> = ({ column, onClose, onSave }) => {

    const parseInitialType = (): 'todo' | 'wip' | 'done' => {
        if (!column) return 'todo';
        return column.kind.type;
    };

    const parseInitialLimit = (): number | '' => {
        if (column && (column.kind.type === 'wip' || column.kind.type === 'todo')) {
            return column.kind.limit !== null ? column.kind.limit : '';
        }
        return '';
    };

    const [name, setName] = useState(column?.name || '');
    const [type, setType] = useState<'todo' | 'wip' | 'done'>(parseInitialType());
    const [limit, setLimit] = useState<number | ''>(parseInitialLimit());
    const [isSaving, setIsSaving] = useState(false);

    const handleSave = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        setIsSaving(true);

        let kind: ColumnType;
        const parsedLimit = limit === '' ? null : Number(limit);

        if (type === 'wip') kind = { type: 'wip', limit: parsedLimit };
        else if (type === 'todo') kind = { type: 'todo', limit: parsedLimit };
        else kind = { type: 'done' };

        await onSave(name, kind);
        setIsSaving(false);
        onClose();
    };

    return (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4">
            <div className="bg-white rounded-xl shadow-2xl w-full max-w-sm overflow-hidden flex flex-col">
                <div className="flex justify-between items-center p-6 border-b border-gray-100">
                    <h2 className="text-xl font-bold text-gray-900">{column ? 'Edit Column' : 'New Column'}</h2>
                    <button onClick={onClose} className="text-gray-400 hover:text-gray-700 text-2xl leading-none">&times;</button>
                </div>

                <form id="column-form" onSubmit={handleSave} className="p-6 space-y-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Column Name</label>
                        <input type="text" autoFocus required value={name} onChange={e => setName(e.target.value)} className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 outline-none" placeholder="e.g. Quality Assurance" />
                    </div>

                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Column Type</label>
                        <select value={type} onChange={e => setType(e.target.value as 'todo' | 'wip' | 'done')} className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 outline-none bg-white">
                            <option value="todo">To Do (Backlog)</option>
                            <option value="wip">In Progress (WIP)</option>
                            <option value="done">Done (Completed)</option>
                        </select>
                    </div>

                    {(type === 'wip' || type === 'todo') && (
                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-1">Item Limit (Optional)</label>
                            <input type="number" min="1" value={limit} onChange={e => setLimit(e.target.value === '' ? '' : parseInt(e.target.value))} className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 outline-none" placeholder="No limit" />
                            <p className="text-xs text-gray-500 mt-1">Leave empty for unlimited items.</p>
                        </div>
                    )}
                </form>

                <div className="p-4 border-t border-gray-100 bg-gray-50 flex justify-end gap-3">
                    <button type="button" onClick={onClose} className="px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-200 rounded-lg">Cancel</button>
                    <button form="column-form" type="submit" disabled={isSaving || !name.trim()} className="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg disabled:opacity-50">
                        {isSaving ? 'Saving...' : 'Save Column'}
                    </button>
                </div>
            </div>
        </div>
    );
};