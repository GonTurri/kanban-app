export type BoardRole = 'owner' | 'editor' | 'viewer';
export type ItemPriority = 'low' | 'medium' | 'high';

export type ColumnType =
    | { type: 'todo'; limit: number | null }
    | { type: 'wip'; limit: number | null }
    | { type: 'done' };

export interface Item {
    id: string;
    title: string;
    description?: string;
    priority: ItemPriority;
    assigned_to?: string;
    is_done: boolean;
    created_at: string;
}

export interface Column {
    id: string;
    name: string;
    order_index: number;
    kind: ColumnType;
    items: Item[];
}

export interface BoardMember {
    id: string;
    user_id: string;
    email: string;
    user_name: string;
    role: BoardRole;
}

export interface Board {
    id: string;
    title: string;
    description: string;
    owner_id: string;
    members: BoardMember[];
    columns: Column[];
}

export interface ItemMetrics {
    lead_time_hours: number;
    cycle_time_hours: number;
}

export interface BoardSummary {
    id: string;
    title: string;
    description: string;
    role: string;
}

export interface ItemHistory {
    id: string;
    item_id: string;
    prev_column_id: string | null;
    new_column_id: string;
    timestamp: string;
}