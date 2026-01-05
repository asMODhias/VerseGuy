import React from 'react'
import { Meta, Story } from '@storybook/react'
import MemberList, { Member } from './MemberList'

export default {
  title: 'Components/MemberList',
  component: MemberList
} as Meta

const Template: Story<{ members: Member[] }> = (args) => <MemberList {...args} />

export const Empty = Template.bind({})
Empty.args = { members: [] }

export const WithMembers = Template.bind({})
WithMembers.args = {
  members: [
    { id: '1', name: 'Alice', role: 'Leader' },
    { id: '2', name: 'Bob' }
  ]
}
