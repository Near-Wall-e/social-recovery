import React, { useState } from 'react'
import { Radio, RadioGroup, Stack } from "@chakra-ui/react";


export default function RadioExample() {
    const [value, setValue] = useState('1')
    return (
        <RadioGroup onChange={setValue} value={value}>
            <Stack direction='row'>
                <Radio value='1'>3 days</Radio>
                <Radio value='2'>Week</Radio>
                <Radio value='3'>Month</Radio>
            </Stack>
        </RadioGroup>
    )
}