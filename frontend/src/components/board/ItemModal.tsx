import React, { useState, useEffect } from 'react';
import api from '../../api/axios';
import type { Item, ItemHistory, ItemMetrics, ItemPriority, Column, BoardMember } from '../../types';

interface ItemModalProps {
    item: Item;
    columns: Column[];
    members: BoardMember[];
    canEdit: boolean;
    onClose: () => void;
    onUpdate: (itemId: string, title: string, desc: string | undefined, prio: ItemPriority, assignedTo: string | null) => Promise<void>;
    onDelete: (itemId: string) => Promise<void>;
}

export const ItemModal: React.FC<ItemModalProps> = ({ item, columns, members, canEdit, onClose, onUpdate, onDelete }) => {
    const [title, setTitle] = useState(item.title);
    const [description, setDescription] = useState(item.description || '');
    const [priority, setPriority] = useState<ItemPriority>(item.priority);
    const [assignedTo, setAssignedTo] = useState<string>(item.assigned_to || '');

    const [history, setHistory] = useState<ItemHistory[]>([]);
    const [metrics, setMetrics] = useState<ItemMetrics | null>(null);
    const [showHistory, setShowHistory] = useState(false);
    const [isLoadingHistory, setIsLoadingHistory] = useState(false);
    const [isSaving, setIsSaving] = useState(false);

    useEffect(() => {
        let isMounted = true;
        const fetchMetrics = async () => {
            if (!item.is_done) return;
            try {
                const res = await api.get<ItemMetrics | null>(`/items/${item.id}/metrics`);
                if (isMounted) setMetrics(res.data);
            } catch { /* empty */ }
        };
        fetchMetrics();
        return () => { isMounted = false; };
    }, [item.id, item.is_done]);

    const handleLoadHistory = async () => {
        setIsLoadingHistory(true);
        try {
            const res = await api.get<ItemHistory[]>(`/items/${item.id}/history`);
            const sortedHistory = res.data.sort((a, b) => new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime());
            setHistory(sortedHistory);
            setShowHistory(true);
        } catch {
            console.error("Failed to load history");
        } finally {
            setIsLoadingHistory(false);
        }
    };

    const handleSave = async (e: React.SubmitEvent) => {
        e.preventDefault();
        if (!canEdit) return;
        setIsSaving(true);
        await onUpdate(item.id, title, description || undefined, priority, assignedTo || null);
        setIsSaving(false);
        onClose();
    };

    const handleDelete = async () => {
        if (!canEdit) return;
        if (window.confirm("Are you sure you want to delete this task?")) {
            await onDelete(item.id);
            onClose();
        }
    };

    const getColumnName = (colId: string | null) => {
        if (!colId) return 'Creation';
        const col = columns.find(c => c.id === colId);
        return col ? col.name : 'Unknown Column';
    };

    const formatDecimalHours = (decimalHours: number) => {
        const totalMinutes = Math.round(decimalHours * 60);

        const days = Math.floor(totalMinutes / (24 * 60));
        const hours = Math.floor((totalMinutes % (24 * 60)) / 60);
        const minutes = totalMinutes % 60;

        const parts = [];

        if (days > 0) parts.push(`${days}d`);
        if (hours > 0) parts.push(`${hours}h`);
        if (minutes > 0 || parts.length === 0) parts.push(`${minutes}m`);

        return parts.join(' ');
    };

    return (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4">
            <div className="bg-white rounded-xl shadow-2xl w-full max-w-3xl max-h-[90vh] overflow-hidden flex flex-col">
                <div className="flex justify-between items-center p-6 border-b border-gray-100">
                    <h2 className="text-xl font-bold text-gray-900">Task Details</h2>
                    <button onClick={onClose} className="text-gray-400 hover:text-gray-700 text-2xl leading-none">&times;</button>
                </div>

                <div className="p-6 overflow-y-auto flex-1 flex flex-col md:flex-row gap-8">
                    <form id="edit-form" onSubmit={handleSave} className="flex-1 space-y-4">
                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-1">Title</label>
                            <input type="text" required disabled={!canEdit} value={title} onChange={e => setTitle(e.target.value)} className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 outline-none disabled:bg-gray-50 disabled:text-gray-500" />
                        </div>

                        <div className="grid grid-cols-2 gap-4">
                            <div>
                                <label className="block text-sm font-medium text-gray-700 mb-1">Priority</label>
                                <select value={priority} disabled={!canEdit} onChange={e => setPriority(e.target.value as ItemPriority)} className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 outline-none bg-white disabled:bg-gray-50 disabled:text-gray-500">
                                    <option value="low">Low</option>
                                    <option value="medium">Medium</option>
                                    <option value="high">High</option>
                                </select>
                            </div>
                            <div>
                                <label className="block text-sm font-medium text-gray-700 mb-1">Assigned To</label>
                                <select value={assignedTo} disabled={!canEdit} onChange={e => setAssignedTo(e.target.value)} className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 outline-none bg-white disabled:bg-gray-50 disabled:text-gray-500">
                                    <option value="">Unassigned</option>
                                    {members.map(m => (
                                        <option key={m.user_id} value={m.user_id}>{m.user_name} ({m.email})</option>
                                    ))}
                                </select>
                            </div>
                        </div>

                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-1">Description</label>
                            <textarea rows={4} disabled={!canEdit} value={description} onChange={e => setDescription(e.target.value)} className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 outline-none resize-none disabled:bg-gray-50 disabled:text-gray-500" placeholder="Add details..." />
                        </div>
                    </form>

                    <div className="w-full md:w-64 flex flex-col gap-6">
                        {metrics ? (
                            <div className="bg-green-50 p-4 rounded-lg border border-green-100">
                                <h3 className="text-sm font-bold text-green-800 mb-2">Metrics</h3>
                                <div className="text-xs text-green-700 space-y-1">
                                    <p>Lead Time: <span className="font-semibold">{formatDecimalHours(metrics.lead_time_hours)}</span></p>
                                    <p>Cycle Time: <span className="font-semibold">{formatDecimalHours(metrics.cycle_time_hours)}</span></p>
                                </div>
                            </div>
                        ) : item.is_done ? (
                            <div className="text-xs text-gray-500 italic">Calculating metrics...</div>
                        ) : (
                            <div className="bg-gray-50 p-4 rounded-lg border border-gray-100 text-xs text-gray-500 text-center">
                                Metrics available when Done.
                            </div>
                        )}

                        <div>
                            <h3 className="text-sm font-bold text-gray-900 mb-3">History</h3>
                            {!showHistory ? (
                                <button type="button" onClick={handleLoadHistory} disabled={isLoadingHistory} className="w-full py-2 text-sm font-medium text-blue-600 bg-blue-50 hover:bg-blue-100 rounded-lg">
                                    {isLoadingHistory ? 'Loading...' : 'Load History'}
                                </button>
                            ) : (
                                <div className="space-y-3 max-h-48 overflow-y-auto pr-2">
                                    {history.map((h) => (
                                        <div key={h.id} className="text-xs border-l-2 border-blue-200 pl-3 py-1">
                                            <p className="text-gray-600 mb-0.5">Moved to <span className="font-semibold">{getColumnName(h.new_column_id)}</span></p>
                                            <p className="text-gray-400 text-[10px]">{new Date(h.timestamp).toLocaleString()}</p>
                                        </div>
                                    ))}
                                </div>
                            )}
                        </div>
                    </div>
                </div>

                <div className="p-4 border-t border-gray-100 bg-gray-50 flex justify-between gap-3">
                    {canEdit ? (
                        <>
                            <button type="button" onClick={handleDelete} className="px-4 py-2 text-sm font-medium text-red-600 hover:bg-red-50 rounded-lg">Delete Task</button>
                            <div className="flex gap-3">
                                <button type="button" onClick={onClose} className="px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-200 rounded-lg">Cancel</button>
                                <button form="edit-form" type="submit" disabled={isSaving} className="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg">Save Changes</button>
                            </div>
                        </>
                    ) : (
                        <div className="w-full flex justify-end">
                            <button type="button" onClick={onClose} className="px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-200 rounded-lg">Close</button>
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
};