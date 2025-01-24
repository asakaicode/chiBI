import { Metadata } from 'next'
import './globals.css'

export const metadata: Metadata = {
  title: 'chiBI',
  description: 'chiBIでBIを楽しもう',
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="ja">
      <body>{children}</body>
    </html>
  )
}
