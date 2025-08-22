
import AuthButton from './components/AuthButton';
import Navbar from './components/Navbar';

export default function Home() {
  return (
    <main className="page-background">
      <Navbar />
      <AuthButton />
    </main>
  );
}