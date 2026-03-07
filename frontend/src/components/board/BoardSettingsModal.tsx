import React, { useState } from 'react';
import type { Board, BoardRole } from '../../types';

interface BoardSettingsModalProps {
    board: Board;
    canManage: boolean;
    onClose: () => void;
    onAddMember: (email: string, role: BoardRole) => Promise<void>;
    onUpdateRole: (userId: string, role: BoardRole) => Promise<void>;
    onRemoveMember: (userId: string) => Promise<void>;
}

export const BoardSettingsModal: React.FC<BoardSettingsModalProps> = ({ board, canManage, onClose, onAddMember, onUpdateRole, onRemoveMember }) => {
    const [newEmail, setNewEmail] = useState('');
    const [newRole, setNewRole] = useState<BoardRole>('viewer');
    const [isInviting, setIsInviting] = useState(false);
    const [inviteError, setInviteError] = useState('');

    const handleInvite = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        setInviteError('');
        setIsInviting(true);
        try {
            await onAddMember(newEmail, newRole);
            setNewEmail('');
        } catch {
            setInviteError('Could not add member. Verify the email exists.');
        } finally {
            setIsInviting(false);
        }
    };

    const handleRoleChange = (userId: string, userName: string, newRole: BoardRole) => {
        if (window.confirm(`Are you sure you want to change ${userName}'s role to ${newRole}?`)) {
            onUpdateRole(userId, newRole);
        }
    };

    const handleRemoveMember = (userId: string, userName: string) => {
        if (window.confirm(`Are you sure you want to remove ${userName} from this board?`)) {
            onRemoveMember(userId);
        }
    };

    return (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4">
            <div className="bg-white rounded-xl shadow-2xl w-full max-w-2xl max-h-[90vh] flex flex-col">
                <div className="flex justify-between items-center p-6 border-b border-gray-100">
                    <div>
                        <h2 className="text-xl font-bold text-gray-900">Board Settings</h2>
                        <p className="text-sm text-gray-500">Manage members and permissions</p>
                    </div>
                    <button onClick={onClose} className="text-gray-400 hover:text-gray-700 text-2xl leading-none">&times;</button>
                </div>

                <div className="p-6 overflow-y-auto flex-1 space-y-8">
                    {canManage && (
                        <div className="bg-gray-50 p-4 rounded-lg border border-gray-200">
                            <h3 className="text-sm font-semibold text-gray-900 mb-3">Invite New Member</h3>
                            <form onSubmit={handleInvite} className="flex gap-3 items-start">
                                <div className="flex-1">
                                    <input type="email" required placeholder="user@example.com" value={newEmail} onChange={e => setNewEmail(e.target.value)} className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 outline-none text-sm" />
                                    {inviteError && <p className="text-red-500 text-xs mt-1">{inviteError}</p>}
                                </div>
                                <select value={newRole} onChange={e => setNewRole(e.target.value as BoardRole)} className="px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 outline-none bg-white text-sm">
                                    <option value="viewer">Viewer</option>
                                    <option value="editor">Editor</option>
                                    <option value="owner">Owner</option>
                                </select>
                                <button type="submit" disabled={isInviting || !newEmail} className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 text-sm font-medium">
                                    {isInviting ? 'Sending...' : 'Invite'}
                                </button>
                            </form>
                        </div>
                    )}

                    <div>
                        <h3 className="text-sm font-semibold text-gray-900 mb-3">Current Members ({board.members.length})</h3>
                        <div className="space-y-3">
                            {board.members.map(member => (
                                <div key={member.id} className="flex items-center justify-between p-3 border border-gray-100 rounded-lg bg-white">
                                    <div>
                                        <p className="font-medium text-gray-900 text-sm">{member.user_name}</p>
                                        <p className="text-xs text-gray-500">{member.email}</p>
                                    </div>
                                    <div className="flex items-center gap-3">
                                        <select
                                            value={member.role}
                                            onChange={e => handleRoleChange(member.user_id, member.user_name, e.target.value as BoardRole)}
                                            disabled={!canManage || member.user_id === board.owner_id}
                                            className="px-2 py-1 border border-gray-300 rounded focus:ring-2 focus:ring-blue-500 outline-none bg-white text-xs disabled:bg-gray-50 disabled:text-gray-500"
                                        >
                                            <option value="viewer">Viewer</option>
                                            <option value="editor">Editor</option>
                                            <option value="owner">Owner</option>
                                        </select>
                                        {canManage && (
                                            <button
                                                onClick={() => handleRemoveMember(member.user_id, member.user_name)}
                                                disabled={member.user_id === board.owner_id}
                                                className="text-red-500 hover:text-red-700 text-xs font-medium disabled:opacity-30"
                                            >
                                                Remove
                                            </button>
                                        )}
                                    </div>
                                </div>
                            ))}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};