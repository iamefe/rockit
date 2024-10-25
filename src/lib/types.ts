export interface Task {
    id: number;
    description: string;
    completed: boolean;
    deleting: boolean;
    created_at: Date;
}