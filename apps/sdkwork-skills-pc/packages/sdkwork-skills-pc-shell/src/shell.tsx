import { NavLink, Outlet } from 'react-router-dom';

const links = [
  { to: '/skills-hub', label: 'Skills Hub' },
  { to: '/console/skills', label: 'Console' },
  { to: '/admin/skills', label: 'Admin Skills' },
  { to: '/admin/categories', label: 'Admin Categories' },
];

export function SkillsShell() {
  return (
    <div style={{ fontFamily: 'system-ui, sans-serif', padding: 24 }}>
      <header style={{ marginBottom: 24 }}>
        <h1>SDKWork Skills</h1>
        <nav style={{ display: 'flex', gap: 16 }}>
          {links.map((link) => (
            <NavLink key={link.to} to={link.to}>
              {link.label}
            </NavLink>
          ))}
        </nav>
      </header>
      <main>
        <Outlet />
      </main>
    </div>
  );
}
