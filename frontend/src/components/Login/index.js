import "regenerator-runtime/runtime"
import React, { useEffect } from "react"
import { useNavigate } from "react-router-dom";
import { useWalletSelector } from "../../context/WalletSelector";
import { setupModal } from "@near-wallet-selector/modal-ui";
import "./index.css"
import { Box, Button, Heading } from "@chakra-ui/react";

const mnwIcon = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABQAAAAYCAYAAAD6S912AAAACXBIWXMAAAsTAAALEwEAmpwYAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAPpSURBVHgBrZQ/bBxFFMbfm9nd20sgWgoKS0heqogC6VwghESR7YAmcQcF4q6k4lyAkiqXAhHRJA0K3TlVJJpLg1yuUyCX5y4STTYVNIhDvj++/TOPb2b3zmc76TLyaHblnd9933vfDI+6EoVEHSLKPt/njC6M0QuJWgvTXRLfPDXSWYqJckOTJdPxopLHP+wE+5vf8+9fy0iIbhqml5WiZHcDevBcbgjLqDAUzY2hpQgtjazXHBsLkYwKSgaftN0+ZYhuVUxsFMVCkj7pSrwC4n34ephYmJ3xQsv4+z/yjgOKwkbMCi8Vc6wUpcMGCpvJHApOzSaMGmVOHZVYK6Go9MtRf/xvpE5zEgszLJhYieJAi4PufsDZaZ4n+CRbCupWmXu5mB5A+6hj5mB2L7wUlYmnU6/PP38q8t42kW45WKNWoJYyKTjpoaYPxguneG+nva7v7aNFXMCqiKBJQvOCaF7KIf/4sYgP2Na2kArIqawcmEkAZaak9+vl7tvRPzoZlMR3LWyKYs8LmSjt1f/852+msqqBFmZtoAzxUkn6qH/WqM2B2mYzwE7gf+agFKkWQmin5xMtJgAap8yBS2vf5zgH9Kfbl6GzvFY2Q4emgNqpggbYagkFPtvW3ithtdKAQX2hjYMqT9LB4Dz0xKpCy6eFm3IGbBMFbSYfz4FXZXCeAJYBBChT4aFJyCmr89BZUTnQDNBZbtjW0lleq8T0Q03fogm54gRdzEqvVlqvEpdXKe0/qKEnOTnQNK9t226vLfuhOKVhWLlf33tYQ6EyKxv7VjHUxsqXtP9IYqfQqittDsXtU85uaEHswCEUrsad+4Bybb9sVJbWvoZShL9V+NvIHpU2wMhXDWwUBs302ySbhR8MGFm0SqmxX9e19CW+Xl0ZePYkkDv4hJDXltcwdDoML+dtcMcGmxNYzwqPxCmFkcDX9NGVaxQyoOz+LFAmdR0BQ6eD0LxPrxj396C0zdb+S9ilOgHIL9R+GL1NodbOtlV4HLiGWBiy2OLvng/P5+0F3u182KuVukahloVVCrDX0nL93bcQOUU8/kXuaiUDW1NcXaRx7OAg04oOsT7zNG2jNH2sE2hItr7irPtE4jyUtPQpXsUKEaM5DiyPhxIFhYwBwl2IogJowcxnz+uJGuJIJlu7nH15gHOOC9k00MKVoa4j/TmUDvam2BC5jVynAO9QXD/bEq2gktPOO7s8uXVQK3WBb3LKm3XSIaA4YqoBcAN3pdC4M5j+w6e9a1/w09W+z1KJizaldTbNM77Yzb9+k67v0zeAdQCyiieAHrNHT6/O6DFD2cU9N44krvyqo7U+pDc9/gd62lAtij23TgAAAABJRU5ErkJggg==";

export default function Login() {
  const { modal, accounts } = useWalletSelector();
  const navigate = useNavigate();

  const handleLogin = async () => {
    modal.show()
  }

  useEffect(() => {
    if (accounts[0]) {
      navigate("/recovery")
    }
  }, [accounts])

  return (
      <div className="login-page">
        <Box
            p={ 8 }
            gap={ 4 }
            maxW="sm"
            borderWidth="2px"
            borderRadius="lg"
            overflow="hidden"
            boxShadow="xl"
            display="flex"
            alignItems="center"
            justifyContent="center"
            flexDir="column"
        >
          <Heading
              as="h1"
              size="md"
          >
            Wall-e - save your assets
          </Heading>
          <Button
              colorScheme="twitter"
              className="login-button"
              onClick={ handleLogin }
          >
            <img
                src={ mnwIcon }
                className="login-button-icon"
                alt=""
            />
            Sign in
          </Button>
        </Box>
      </div>
  )
}
