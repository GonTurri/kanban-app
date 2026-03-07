import { Routes, Route, Navigate } from 'react-router-dom';
import { Login } from './pages/Login';
import { Register } from './pages/Register';
import { Dashboard } from './pages/Dashboard';
import { ProtectedRoute } from './components/auth/ProtectedRoute';
import { useAuth } from './context/AuthContext';
import { useSilentRefresh } from './hooks/useSilentRefresh';

function App() {
    const { isAuthenticated } = useAuth();
    useSilentRefresh(isAuthenticated);

    return (
        <Routes>
            <Route path="/login" element={<Login />} />
            <Route path="/register" element={<Register />} />

            <Route element={<ProtectedRoute />}>
                <Route path="/" element={<Dashboard />} />
                <Route path="/board/:boardId" element={<Dashboard />} />
            </Route>

            <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
    );
}

export default App;