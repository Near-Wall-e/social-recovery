import { Button, Grid, GridItem } from "@chakra-ui/react";
import { useWalletSelector } from "../../context/WalletSelector";
import React from "react"
import { FaSignOutAlt } from "react-icons/fa";
import "./index.css"

export function PageHeader() {
  const { selector, accountId } = useWalletSelector();

  const signOut = async () => {
    const wallet = await selector.wallet();
    wallet.signOut()
  }

  return (
      <Grid
          height={ "100%" }
          gap={ 4 }
          p={2}
          templateColumns='repeat(3, 1fr)'
      >
        <GridItem
            colSpan={ 1 }
        />
        <GridItem
            colSpan={ 1 }
        />
        <GridItem
            colSpan={ 1 }
            className="header-profile"
        >
          <p className="account-name">{ accountId }</p>
          <Button
              variant="solid"
              colorScheme="orange"
              onClick={ signOut }
          >
            <FaSignOutAlt className="signout-icon" />
            Sign Out
          </Button>
        </GridItem>
      </Grid>
  )
}
