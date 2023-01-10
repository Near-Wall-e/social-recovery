import { Slider, SliderFilledTrack, SliderMark, SliderThumb, SliderTrack, Tooltip } from "@chakra-ui/react";
import React, { useState } from "react";
import './index.css'

export default function AddThreshold({ wallets }) {
    const [sliderValue, setSliderValue] = React.useState(1)
    const [showTooltip, setShowTooltip] = React.useState(false)

    const labelStyles = {
        mt: '2',
        ml: '-2.5',
        fontSize: 'sm',
    }

    return (
        <Slider
            id='slider'
            defaultValue={1}
            min={1}
            max={wallets}
            step={1}
            colorScheme='teal'
            onChange={(v) => setSliderValue(v)}
            onMouseEnter={() => setShowTooltip(true)}
            onMouseLeave={() => setShowTooltip(false)}
        >
            <SliderMark value={1} mt='1' ml='-2.5' fontSize='sm'>
                1
            </SliderMark>
            <SliderMark value={wallets} mt='1' ml='-2.5' fontSize='sm'>
                { wallets }
            </SliderMark>
            <SliderTrack>
                <SliderFilledTrack />
            </SliderTrack>
            <Tooltip
                hasArrow
                bg='teal.500'
                color='white'
                placement='top'
                isOpen={showTooltip}
                label={sliderValue}
            >
                <SliderThumb />
            </Tooltip>
        </Slider>
    )
}