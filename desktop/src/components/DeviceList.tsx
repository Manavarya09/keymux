import React from 'react'

export default function DeviceList({ devices }: { devices: any[] }) {
  return (
    <ul>
      {devices.map((d, i) => (
        <li key={i}>{d.name}</li>
      ))}
    </ul>
  )
}
