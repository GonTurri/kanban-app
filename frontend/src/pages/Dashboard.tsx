import React, { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';
import { useBoard } from '../hooks/useBoard';
import { BoardView } from '../components/board/BoardView';
import { CreateBoardModal } from '../components/board/CreateBoardModal';

export const Dashboard: React.FC = () => {
    const { user, logout } = useAuth();
    const { boardId } = useParams<{ boardId: string }>();
    const navigate = useNavigate();
    const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);

    const {
        board, boardsList, isLoading, error, fetchBoard, fetchMyBoards, createBoard, closeBoard,
        addItem, moveItem, updateItemDetails, deleteItem, loadMoreItems,
        addMember, updateMemberRole, removeMember,
        addColumn, updateColumn, deleteColumn, moveColumn
    } = useBoard();

    useEffect(() => {
        if (boardId) {
            fetchBoard(boardId);
        } else {
            closeBoard();
            fetchMyBoards();
        }
    }, [boardId, fetchBoard, fetchMyBoards, closeBoard]);

    const handleCreateBoard = async (title: string, description: string) => {
        const newBoardId = await createBoard(title, description);
        if (newBoardId) {
            setIsCreateModalOpen(false);
            navigate(`/board/${newBoardId}`);
        }
    };

    return (
        <div className="flex flex-col h-screen bg-gray-50">
            <header className="flex items-center justify-between px-6 py-4 bg-white border-b border-gray-200 shadow-sm z-10">
                <div className="flex items-center gap-4">
                    <div className="w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center text-white font-bold cursor-pointer" onClick={() => navigate('/')}>
                        K
                    </div>
                    <div>
                        <h1 className="text-lg font-bold text-gray-900 leading-tight">{board ? board.title : 'My Boards'}</h1>
                        <p className="text-xs text-gray-500">Logged in as {user?.email}</p>
                    </div>
                </div>
                <div className="flex items-center gap-4">
                    {board && (
                        <button onClick={() => navigate('/')} className="px-4 py-2 text-sm font-medium text-gray-600 hover:bg-gray-100 rounded-lg transition-colors">
                            Back to Boards
                        </button>
                    )}
                    <button onClick={logout} className="px-4 py-2 text-sm font-medium text-red-600 bg-red-50 hover:bg-red-100 rounded-lg transition-colors">
                        Sign Out
                    </button>
                </div>
            </header>

            <main className="flex-1 overflow-hidden relative">
                {error && (
                    <div className="absolute top-4 left-1/2 transform -translate-x-1/2 z-50 min-w-xs max-w-md p-4 bg-red-50 text-red-700 border border-red-200 shadow-lg rounded-xl text-sm text-center font-medium animate-bounce">
                        {error}
                    </div>
                )}

                {!board ? (
                    <div className="p-8 h-full overflow-y-auto">
                        <div className="max-w-6xl mx-auto">
                            <div className="flex justify-between items-center mb-6">
                                <h2 className="text-2xl font-bold text-gray-900">Your Workspaces</h2>
                                <button
                                    onClick={() => setIsCreateModalOpen(true)}
                                    disabled={isLoading}
                                    className="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 font-medium transition-colors disabled:opacity-50"
                                >
                                    + Create New Board
                                </button>
                            </div>

                            {isLoading && boardsList.length === 0 ? (
                                <p className="text-gray-500">Loading your boards...</p>
                            ) : boardsList.length === 0 ? (
                                <div className="text-center py-12 bg-white rounded-xl border border-gray-200 border-dashed">
                                    <p className="text-gray-500 mb-4">You don't have any boards yet.</p>
                                </div>
                            ) : (
                                <div className="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-4 gap-6">
                                    {boardsList.map((b) => (
                                        <div
                                            key={b.id}
                                            onClick={() => navigate(`/board/${b.id}`)}
                                            className="bg-white p-5 rounded-xl border border-gray-200 shadow-sm hover:shadow-md hover:border-blue-300 transition-all cursor-pointer group"
                                        >
                                            <h3 className="font-bold text-gray-900 text-lg mb-1 group-hover:text-blue-600">{b.title}</h3>
                                            <p className="text-sm text-gray-500 line-clamp-2 mb-4">{b.description}</p>
                                            <span className="inline-block px-2.5 py-1 bg-gray-100 text-gray-600 text-xs font-semibold rounded-md">Role: {b.role}</span>
                                        </div>
                                    ))}
                                </div>
                            )}
                        </div>
                    </div>
                ) : (
                    <BoardView
                        board={board}
                        onMoveItem={moveItem}
                        onAddItem={addItem}
                        onUpdateItem={updateItemDetails}
                        onDeleteItem={deleteItem}
                        onLoadMoreItems={loadMoreItems}
                        onAddMember={addMember}
                        onUpdateMemberRole={updateMemberRole}
                        onRemoveMember={removeMember}
                        onAddColumn={addColumn}
                        onUpdateColumn={updateColumn}
                        onDeleteColumn={deleteColumn}
                        onMoveColumn={moveColumn}
                    />
                )}

                {isCreateModalOpen && (
                    <CreateBoardModal
                        onClose={() => setIsCreateModalOpen(false)}
                        onSave={handleCreateBoard}
                    />
                )}
            </main>
        </div>
    );
};