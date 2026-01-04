import React from 'react'
import { Meta, Story } from '@storybook/react'
import LicenseGuard from './LicenseGuard'

export default {
  title: 'Components/LicenseGuard',
  component: LicenseGuard
} as Meta

const Template: Story<any> = (args) => <LicenseGuard {...args}><div>Protected Content</div></LicenseGuard>

export const RequiresPro = Template.bind({})
RequiresPro.args = { required: 'Pro' }

export const RequiresFree = Template.bind({})
RequiresFree.args = { required: 'Free' }
