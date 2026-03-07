import { useEffect } from 'react';
import api from '../api/axios';

export const useSilentRefresh = (isAuthenticated: boolean) => {
    useEffect(() => {
        if (!isAuthenticated) return;

        const envMinutes = Number(import.meta.env.VITE_REFRESH_INTERVAL_MINUTES);
        const intervalMinutes = isNaN(envMinutes) || envMinutes <= 0 ? 10 : envMinutes;

        const REFRESH_INTERVAL_MS = intervalMinutes * 60 * 1000;

        const intervalId = setInterval(async () => {
            try {
                await api.post('/auth/refresh');
                console.debug(`Silent refresh executed successfully (Interval: ${intervalMinutes}m)`);
            } catch (error) {
                console.error('Failed el silent refresh', error);
            }
        }, REFRESH_INTERVAL_MS);

        return () => clearInterval(intervalId);
    }, [isAuthenticated]);
};