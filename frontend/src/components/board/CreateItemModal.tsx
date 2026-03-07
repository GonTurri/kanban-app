import React, { useState } from 'react';
import type { ItemPriority, BoardMember } from '../../types';

interface CreateItemModalProps {
    columnName: string;
    members: BoardMember[];
    onClose: () => void;
    onSave: (title: string, description: string | undefined, priority: ItemPriority, assignedTo: string | null) => Promise<void>;
}

export const CreateItemModal: React.FC<CreateItemModalProps> = ({ columnName, members, onClose, onSave }) => {
    const [title, setTitle] = useState('');
    const [description, setDescription] = useState('');
    const [priority, setPriority] = useState<ItemPriority>('medium');
    const [assignedTo, setAssignedTo] = useState<string>('');
    const [isSaving, setIsSaving] = useState(false);

    const handleSave = async (e: React.SubmitEvent) => {
        e.preventDefault();
        setIsSaving(true);
        await onSave(title, description || undefined, priority, assignedTo || null);
        setIsSaving(false);
        onClose();
    };

    return (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4">
            <div className="bg-white rounded-xl shadow-2xl w-full max-w-lg overflow-hidden flex flex-col">
                <div className="flex justify-between items-center p-6 border-b border-gray-100">
                    <h2 className="text-xl font-bold text-gray-900">Add to "{columnName}"</h2>
                    <button onClick={onClose} className="text-gray-400 hover:text-gray-700 text-2xl leading-none">&times;</button>
                </div>

                <form id="create-item-form" onSubmit={handleSave} className="p-6 space-y-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Title</label>
                        <input type="text" autoFocus required value={title} onChange={e => setTitle(e.target.value)} className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 outline-none" placeholder="What needs to be done?" />
                    </div>

                    <div className="grid grid-cols-2 gap-4">
                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-1">Priority</label>
                            <select value={priority} onChange={e => setPriority(e.target.value as ItemPriority)} className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 outline-none bg-white">
                                <option value="low">Low</option>
                                <option value="medium">Medium</option>
                                <option value="high">High</option>
                            </select>
                        </div>
                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-1">Assign To</label>
                            <select value={assignedTo} onChange={e => setAssignedTo(e.target.value)} className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 outline-none bg-white">
                                <option value="">Unassigned</option>
                                {members.map(m => (
                                    <option key={m.user_id} value={m.user_id}>{m.user_name}</option>
                                ))}
                            </select>
                        </div>
                    </div>

                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Description</label>
                        <textarea rows={3} value={description} onChange={e => setDescription(e.target.value)} className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 outline-none resize-none" placeholder="Add more details..." />
                    </div>
                </form>

                <div className="p-4 border-t border-gray-100 bg-gray-50 flex justify-end gap-3">
                    <button type="button" onClick={onClose} className="px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-200 rounded-lg">Cancel</button>
                    <button form="create-item-form" type="submit" disabled={isSaving || !title.trim()} className="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg disabled:opacity-50">
                        {isSaving ? 'Creating...' : 'Create Task'}
                    </button>
                </div>
            </div>
        </div>
    );
};