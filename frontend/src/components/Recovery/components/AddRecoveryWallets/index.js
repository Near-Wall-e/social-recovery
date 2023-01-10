import "regenerator-runtime/runtime"
import React, { useState } from "react"
import "./index.css"
import {
  Button,
  FormControl,
  FormErrorMessage,
  FormHelperText,
  Grid,
  GridItem,
  Heading,
  Input, List, ListIcon, ListItem, Table, TableCaption, TableContainer, Tbody, Td, Tfoot, Th, Thead, Tr
} from "@chakra-ui/react";
import { IoAddOutline } from "react-icons/io5";
import { MdCheckCircle, MdSettings } from "react-icons/md";

export default function AddRecoveryWallets({ wallets, setWallets }) {
  const [wallet, setWallet] = useState({
    value: "",
    isError: false
  });

  const handleWalletChange = (e) => {
    wallet.value = e.target.value;

    setWallet({
      ...wallet,
      value: e.target.value
    })
    // const newWallets = [...wallets]
    // newWallets[index].value = e.target.value
    // setWallets(newWallets)
  }

  const addWallet = () => {
    const newWallets = [...wallets]
    if (newWallets.length < 5) {
      newWallets.push(wallet)
      setWallets(newWallets)
      setWallet({
        value: "",
        isError: false,
      })
    }
  }

  // const deleteWallet = (index) => {
  //   const newWallets = [...wallets]
  //   if (newWallets.length > 1) {
  //     newWallets.splice(index, 1)
  //     setWallets(newWallets)
  //   }
  // }

  return (
      <Grid
          gap={ 2 }
          height="200px"
          templateColumns="repeat(3, 1fr)"
      >
        <GridItem
            colSpan={ 1 }
            rowSpan={ 1 }
        >
        </GridItem>
        <GridItem
            colSpan={ 1 }
            rowSpan={ 1 }
        >
          <Heading
              as="h4"
              size="md"
          >
            Social recovery wallets:
          </Heading>
          <div className="wallet-row">
            <FormControl
                isInvalid={ wallet.isError }
            >
              <Input
                  value={ wallet.value }
                  onChange={ (e) => handleWalletChange(e) }
              />
              {/*<div className="wallets-options">*/ }
              {/*  <Button onClick={ () => deleteWallet(index) }>Delete</Button>*/ }
              {/*</div>*/ }
              { !wallet.isError ? (
                  <FormHelperText>
                    Enter wallet you'd like to be your recovery wallet.
                  </FormHelperText>
              ) : (
                  <FormErrorMessage>Email is required.</FormErrorMessage>
              ) }
            </FormControl>
          </div>
          <Button
              leftIcon={ <IoAddOutline /> }
              colorScheme="gray"
              onClick={ addWallet }
          >
            Add recovery wallet
          </Button>
          <TableContainer>
            <Table
                variant="striped"
                colorScheme="teal"
            >
              {/*<TableCaption>Imperial to metric conversion factors</TableCaption>*/}
              <Thead>
                <Tr>
                  <Th>Wallet name</Th>
                  <Th>Email</Th>
                  <Th>Actions</Th>
                </Tr>
              </Thead>
              <Tbody>
                {
                  wallets.map(wallet => (
                          <Tr>
                            <Td>{ wallet.value }</Td>
                            <Td>email</Td>
                            <Td>kek</Td>
                          </Tr>
                      )
                  )
                }
              </Tbody>
              {/*<Tfoot>*/ }
              {/*  <Tr>*/ }
              {/*    <Th>To convert</Th>*/ }
              {/*    <Th>into</Th>*/ }
              {/*    <Th isNumeric>multiply by</Th>*/ }
              {/*  </Tr>*/ }
              {/*</Tfoot>*/ }
            </Table>
          </TableContainer>
          {/*<List spacing={3}>*/ }
          {/*    <ListItem>*/ }
          {/*        <ListIcon as={MdCheckCircle} color='green.500' />*/ }
          {/*        Lorem ipsum dolor sit amet, consectetur adipisicing elit*/ }
          {/*    </ListItem>*/ }
          {/*</List>*/ }
        </GridItem>
        <GridItem
            colSpan={ 1 }
            rowSpan={ 1 }
        />
      </Grid>
  )
}
