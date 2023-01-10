import 'regenerator-runtime/runtime'
import React from 'react'
import { WalletSelectorContextProvider } from "../../context/WalletSelector";
import { ChakraProvider } from '@chakra-ui/react'
import './index.css'
import Login from "../../components/Login";
// import Loading from "../../components/loading";

export default function LoginPage() {
    // const [isLoading, setIsLoading] = useState(false)
    return (
        <WalletSelectorContextProvider>
            <ChakraProvider>
                <Login />
            </ChakraProvider>
        </WalletSelectorContextProvider>
    )
}
