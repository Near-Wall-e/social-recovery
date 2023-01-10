import React, { useCallback, useContext, useEffect, useState } from "react";
import { map, distinctUntilChanged } from "rxjs";
import { setupWalletSelector } from "@near-wallet-selector/core";
import { setupModal } from "@near-wallet-selector/modal-ui";
import { createAction } from "@near-wallet-selector/wallet-utils";
import { setupMyNearWallet } from "@near-wallet-selector/my-near-wallet";
import { setupNearWallet } from "@near-wallet-selector/near-wallet";
import { connect, keyStores } from "near-api-js";
// import { CONTRACT_ID } from "../constants";

// const CONTRACT_ID = 'kiskesis.testnet'

const WalletSelectorContext =
    React.createContext(null);

export const WalletSelectorContextProvider = ({ children }) => {
    const [selector, setSelector] = useState(null);
    const [modal, setModal] = useState(null);
    const [accounts, setAccounts] = useState([]);
    const [account, setAccount] = useState(null);

    const init = useCallback(async () => {
        const _selector = await setupWalletSelector({
            network: "testnet",
            debug: true,
            modules: [
                setupNearWallet(),
                setupMyNearWallet(),
            ],
        });

        console.log('accountRecovery.testnet');

        const _modal = setupModal(_selector, { contractId: 'walle.testnet' });
        const state = _selector.store.getState();

        setAccounts(state.accounts);
        setSelector(_selector);
        setModal(_modal);

        if (state.accounts[0]) {
            const { network } = _selector.options;

            const connectionConfig = {
                networkId: network.networkId,
                keyStore: new keyStores.BrowserLocalStorageKeyStore(),
                nodeUrl: network.nodeUrl,
                headers: {},
            };

            const nearConnection = await connect(connectionConfig);
            const account = await nearConnection.account(state.accounts[0].accountId);

            setAccount(account)
        }
    }, []);

    useEffect(() => {
        init().catch((err) => {
            console.error(err);
            alert("Failed to initialise wallet selector");
        });
    }, [init]);

    useEffect(() => {
        if (!selector) {
            return;
        }

        const subscription = selector.store.observable
            .pipe(
                map((state) => state.accounts),
                distinctUntilChanged()
            )
            .subscribe((nextAccounts) => {
                console.log("Accounts Update", nextAccounts);

                setAccounts(nextAccounts);
            });

        return () => subscription.unsubscribe();
    }, [selector]);

    if (!selector || !modal) {
        return null;
    }

    const accountId =
        accounts.find((account) => account.active)?.accountId || null;

    return (
        <WalletSelectorContext.Provider
            value={{
                selector,
                modal,
                accounts,
                accountId,
                account
            }}
        >
            {children}
        </WalletSelectorContext.Provider>
    );
};

export function useWalletSelector() {
    const context = useContext(WalletSelectorContext);

    if (!context) {
        throw new Error(
            "useWalletSelector must be used within a WalletSelectorContextProvider"
        );
    }

    return context;
}