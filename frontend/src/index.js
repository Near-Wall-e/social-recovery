import React from "react"
import { createRoot } from "react-dom/client";
import {
    Routes,
    Route,
    BrowserRouter,
} from "react-router-dom";
import './style.css'
import RecoveryPage from "./pages/RecoveryPage";
import LoginPage from "./pages/LoginPage";

const container = document.getElementById('app');
const root = createRoot(container);

root.render(
    <BrowserRouter basename="/">
        <Routes>
            <Route
                path="/"
                element={ <LoginPage /> }
            />
            <Route
                path="/recovery"
                element={ <RecoveryPage /> }
            />
        </Routes>
    </BrowserRouter>
)