import { useState, useCallback } from 'react';
import api from '../api/axios';
import type { Board, ItemPriority, BoardSummary, Item, BoardRole, ColumnType } from '../types';

export const useBoard = () => {
    const [board, setBoard] = useState<Board | null>(null);
    const [boardsList, setBoardsList] = useState<BoardSummary[]>([]);
    const [isLoading, setIsLoading] = useState<boolean>(false);

    const [errorState, setErrorState] = useState<string | null>(null);
    const setError = useCallback((msg: string | null) => {
        setErrorState(msg);
        if (msg) {
            setTimeout(() => setErrorState(null), 5000);
        }
    }, []);

    const fetchMyBoards = useCallback(async () => {
        setIsLoading(true);
        setError(null);
        try {
            const response = await api.get<BoardSummary[]>('/boards');
            setBoardsList(response.data);
        } catch { setError('Failed to load your boards.'); }
        finally { setIsLoading(false); }
    }, [setError]);

    const fetchBoard = useCallback(async (boardId: string) => {
        setIsLoading(true);
        setError(null);
        try {
            const response = await api.get<Board>(`/boards/${boardId}`);
            const fetchedBoard = response.data;
            fetchedBoard.columns.sort((a, b) => a.order_index - b.order_index);
            setBoard(fetchedBoard);
        } catch { setError('Failed to load board details.'); }
        finally { setIsLoading(false); }
    }, [setError]);

    const createBoard = useCallback(async (title: string, description: string) => {
        setIsLoading(true);
        setError(null);
        try {
            const res = await api.post<{ id: string }>('/boards', { title, description });
            return res.data.id;
        } catch {
            setError('Failed to create board.');
            return null;
        } finally { setIsLoading(false); }
    }, [setError]);

    const addItem = useCallback(async (columnId: string, title: string, description?: string, priority: ItemPriority = 'medium', assignedTo: string | null = null) => {
        if (!board) return;
        try {
            await api.post(`/items/column/${columnId}`, { title, description: description || null, priority, assigned_to: assignedTo });
            await fetchBoard(board.id);
        } catch { setError('Failed to add new item.'); }
    }, [board, fetchBoard, setError]);

    const moveItem = useCallback(async (itemId: string, sourceColumnId: string, destinationColumnId: string, sourceIndex: number, destinationIndex: number) => {
        if (!board) return;
        const newBoard = JSON.parse(JSON.stringify(board)) as Board;
        const sourceCol = newBoard.columns.find(c => c.id === sourceColumnId);
        const destCol = newBoard.columns.find(c => c.id === destinationColumnId);
        if (!sourceCol || !destCol) return;

        if (sourceColumnId !== destinationColumnId) {
            const destLimit = (destCol.kind.type === 'wip' || destCol.kind.type === 'todo') ? destCol.kind.limit : null;
            if (destLimit !== null && destCol.items.length >= destLimit) {
                setError(`Cannot move task. "${destCol.name}" has reached its limit of ${destLimit}.`);
                return; // Abort the move entirely
            }
        }

        const [movedItem] = sourceCol.items.splice(sourceIndex, 1);
        destCol.items.splice(destinationIndex, 0, movedItem);
        setBoard(newBoard);

        try {
            if (sourceColumnId !== destinationColumnId) {
                await api.put(`/items/${itemId}/move`, { new_column_id: destinationColumnId });
            }
        } catch {
            setError('Failed to move item.');
            await fetchBoard(board.id);
        }
    }, [board, fetchBoard, setError]);

    const updateItemDetails = useCallback(async (itemId: string, title: string, description: string | undefined, priority: ItemPriority, assigned_to: string | null) => {
        if (!board) return;
        try {
            await api.put(`/items/${itemId}`, { title, description: description || null, priority, assigned_to });
            await fetchBoard(board.id);
        } catch { setError('Failed to update item details.'); }
    }, [board, fetchBoard, setError]);

    const deleteItem = useCallback(async (itemId: string) => {
        if (!board) return;
        try {
            await api.delete(`/items/${itemId}`);
            await fetchBoard(board.id);
        } catch { setError('Failed to delete item.'); }
    }, [board, fetchBoard, setError]);

    const loadMoreItems = useCallback(async (columnId: string, offset: number) => {
        try {
            const response = await api.get<Item[]>(`/items/column/${columnId}?limit=10&offset=${offset}`);
            setBoard(prev => {
                if (!prev) return prev;
                const newBoard = { ...prev };
                const colIndex = newBoard.columns.findIndex(c => c.id === columnId);
                if (colIndex !== -1) {
                    const existingIds = new Set(newBoard.columns[colIndex].items.map(i => i.id));
                    const newItems = response.data.filter(i => !existingIds.has(i.id));
                    newBoard.columns[colIndex].items = [...newBoard.columns[colIndex].items, ...newItems];
                }
                return newBoard;
            });
        } catch { setError('Failed to load more items.'); }
    }, [setError]);

    const addColumn = useCallback(async (name: string, kind: ColumnType, targetIndex: number) => {
        if (!board) return;
        try {
            await api.post(`/columns/board/${board.id}`, { name, kind, target_index: targetIndex });
            await fetchBoard(board.id);
        } catch { setError('Failed to add column.'); }
    }, [board, fetchBoard, setError]);

    const updateColumn = useCallback(async (columnId: string, name: string, kind: ColumnType) => {
        if (!board) return;
        try {
            await api.put(`/columns/${columnId}`, { name, kind });
            await fetchBoard(board.id);
        } catch { setError('Failed to update column.'); }
    }, [board, fetchBoard, setError]);

    const deleteColumn = useCallback(async (columnId: string) => {
        if (!board) return;
        try {
            await api.delete(`/columns/${columnId}`);
            await fetchBoard(board.id);
        } catch { setError('Failed to delete column.'); }
    }, [board, fetchBoard, setError]);

    const moveColumn = useCallback(async (columnId: string, targetIndex: number) => {
        if (!board) return;
        const newBoard = JSON.parse(JSON.stringify(board)) as Board;
        const colIndex = newBoard.columns.findIndex(c => c.id === columnId);
        const [movedCol] = newBoard.columns.splice(colIndex, 1);
        newBoard.columns.splice(targetIndex, 0, movedCol);
        newBoard.columns.forEach((c, idx) => c.order_index = idx);
        setBoard(newBoard);

        try {
            await api.put(`/columns/${columnId}/board/${board.id}/move`, { target_index: targetIndex });
        } catch {
            setError('Failed to move column.');
            await fetchBoard(board.id);
        }
    }, [board, fetchBoard, setError]);

    const addMember = useCallback(async (email: string, role: BoardRole) => {
        if (!board) return;
        try {
            await api.post(`/boards/${board.id}/members`, { email, role });
            await fetchBoard(board.id);
        } catch { throw new Error('Failed to add member'); }
    }, [board, fetchBoard]);

    const updateMemberRole = useCallback(async (userId: string, role: BoardRole) => {
        if (!board) return;
        try {
            await api.put(`/boards/${board.id}/members/${userId}`, { role });
            await fetchBoard(board.id);
        } catch { setError('Failed to update member role.'); }
    }, [board, fetchBoard, setError]);

    const removeMember = useCallback(async (userId: string) => {
        if (!board) return;
        try {
            await api.delete(`/boards/${board.id}/members/${userId}`);
            await fetchBoard(board.id);
        } catch { setError('Failed to remove member.'); }
    }, [board, fetchBoard, setError]);

    const closeBoard = useCallback(() => setBoard(null), []);

    return {
        board, boardsList, isLoading, error: errorState, fetchBoard, fetchMyBoards, createBoard, closeBoard,
        addItem, moveItem, updateItemDetails, deleteItem, loadMoreItems,
        addColumn, updateColumn, deleteColumn, moveColumn,
        addMember, updateMemberRole, removeMember
    };
};