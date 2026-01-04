import React from 'react'
import { Meta, Story } from '@storybook/react'
import RankManagement, { Rank } from './RankManagement'

export default {
  title: 'Components/RankManagement',
  component: RankManagement
} as Meta

const Template: Story<{ ranks: Rank[] }> = (args) => <RankManagement {...args} />

export const Empty = Template.bind({})
Empty.args = { ranks: [] }

export const WithRanks = Template.bind({})
WithRanks.args = { ranks: [{ id: 'r1', name: 'Officer', level: 2 }, { id: 'r2', name: 'Member' }] }
