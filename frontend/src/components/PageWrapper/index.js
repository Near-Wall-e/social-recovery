import { PageHeader } from "../PageHeader";
import React from "react"
import { Grid, GridItem } from "@chakra-ui/react";

export function PageWrapper(props) {
  return (
      <Grid
          height={ "100vh" }
          gap={ 2 }
      >
        <GridItem
            rowSpan={ 1 }
            colSpan={ 1 }
            gap={ 1 }
            bg="aqua"
        >
          <PageHeader />
        </GridItem>
        <GridItem
            rowSpan={ 22 }
            colSpan={ 1 }
            bg="papayawhip"
        >
          { props.children }
        </GridItem>
        <GridItem
            rowSpan={ 1 }
            colSpan={ 1 }
            bg="tomato"
        >
        </GridItem>
      </Grid>
  )
}

