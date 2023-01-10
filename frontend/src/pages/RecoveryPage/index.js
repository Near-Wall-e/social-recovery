import { WalletSelectorContextProvider } from "../../context/WalletSelector";
import { ChakraProvider } from '@chakra-ui/react'
import React from "react";
import { Recovery } from "../../components/Recovery";
import { PageWrapper } from "../../components/PageWrapper";

export default function RecoveryPage() {
    return (
        <WalletSelectorContextProvider>
            <ChakraProvider>
                <PageWrapper>
                    <Recovery />
                </PageWrapper>
            </ChakraProvider>
        </WalletSelectorContextProvider>
    )
}
