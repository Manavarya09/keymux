import React, { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'

type Device = { name: string; vendor_id?: number | null; product_id?: number | null }

export default function App() {
  const [devices, setDevices] = useState<Device[]>([])

  useEffect(() => {
    invoke('list_devices')
      .then((d) => setDevices(d as Device[]))
      .catch(() => setDevices([]))
  }, [])

  return (
    <div className="h-screen w-screen bg-gray-50 text-gray-900 p-6">
      <h1 className="text-2xl font-semibold">Keymux</h1>
      <p className="mt-4">Phase 1: Input Engine (Windows)</p>
      <div className="mt-6">
        <h2 className="text-lg font-medium">Devices</h2>
        <ul className="mt-2 space-y-2">
          {devices.length === 0 ? (
            <li className="text-sm text-gray-500">No devices detected (or running on non-Windows)</li>
          ) : (
            devices.map((d, i) => (
              <li key={i} className="text-sm">
                {d.name} — VID: {d.vendor_id ?? 'n/a'} PID: {d.product_id ?? 'n/a'}
              </li>
            ))
          )}
        </ul>
      </div>
    </div>
  )
}
