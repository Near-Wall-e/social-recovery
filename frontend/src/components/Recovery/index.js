import 'regenerator-runtime/runtime';
import React, { useEffect, useState } from 'react';
import { useNavigate } from "react-router-dom";
import { useWalletSelector } from "../../context/WalletSelector";
import * as NearAPI from "near-api-js";
import './index.css';
import AddRecoveryWallets from "./components/AddRecoveryWallets";
import AddThreeshold from "./components/AddThreeshold";
import AddThreshold from "./components/AddThreeshold";
import { Grid, GridItem } from "@chakra-ui/react";
// import { Buffer } from "buffer";
// import Loading from "../../components/loading";
// import wasmFile from './recovery_contract.wasm'

// const contractFileGis§tURL = 'https://gist.github.com/kiskesis/084ba38ee4981906697d0bbd8cd55601'

export function Recovery() {
    // const [isLoading, setIsLoading] = useState(false)
    const [recoveryContract, setRecoveryContract] = useState(false)
    const [wallets, setWallets] = useState([])
    const { account, accounts, selector, accountId } = useWalletSelector();
    const navigate = useNavigate();

    useEffect(() => {
        console.log('account', account)
    }, [])

    const initContract = async () => {
        setRecoveryContract(await new NearAPI.Contract(account, accountId, {
            changeMethods: ['init'],
            viewMethods: ['get_self'],
        }))
    }

    const deploySocialContract = async () => {
        const contractBytes = new Uint8Array(await (await fetch('/recovery_contract.wasm')).arrayBuffer());

        await account.deployContract(contractBytes)

        await initContract()
        await recoveryContract.init(
            {
                config: {
                    threshold: 1,
                    timeout: 60 * 60 * 24 * 7,
                    accounts: ['kiskesis.testnet', 'walletester.testnet']
                }
            },
            NearAPI.utils.format.parseNearAmount('0.00000000015'),
            1
        )
    }

    useEffect(() => {
        if (!accounts[0]) {
            navigate('/')
        }
    }, [accounts[0]])

    const checkContract = async () => {
        const wallet = await selector.wallet();
        wallet.signOut()
    }

    const initRecovery = async () => {
        const res = await recoveryContract.init({
            config: {
                threshold: 1,
                timeout: 60 * 60 * 24 * 7,
                accounts: ['kiskesis.testnet', 'walletester.testnet']
            }
        })
    }

    const getSelf = async () => {
        console.log('recoveryContract', recoveryContract);
        const res = await recoveryContract.get_self()
        console.log('res', res);
    }

    return (
        <Grid>
            <GridItem className="add-recovery">
                <AddRecoveryWallets wallets={wallets} setWallets={setWallets} />
            </GridItem>
            {/*<GridItem className="add-threshold">*/}
            {/*    <div>Threshold</div>*/}
            {/*    <AddThreshold wallets={wallets.length}/>*/}
            {/*</GridItem>*/}
            {/*<GridItem className="add-timeout">*/}
            {/*    <div>Timeout</div>*/}
            {/*</GridItem>*/}
            {/*<GridItem  className="buttons">*/}
            {/*    <button onClick={ deploySocialContract }>Add recovery</button>*/}
            {/*    /!*<button onClick={ checkContract }>Log Out</button>*!/*/}
            {/*    /!*<button onClick={ initContract }>Init contract</button>*!/*/}
            {/*    /!*<button onClick={ initRecovery }>Init recovery</button>*!/*/}
            {/*    <button onClick={ getSelf }>Get self</button>*/}
            {/*</GridItem>*/}
        </Grid>
    )
}
