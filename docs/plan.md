# Plan de Trabajo — Frontend Airport App (Rust + Dioxus 0.7 + CSS Puro)

**Entrega:** Por definir (sugerencia: 2-3 semanas)
**Tecnología:** Rust + Dioxus 0.7 (Web) + CSS Puro (sin frameworks CSS)
**Backend:** FastAPI (existente) — `https://github.com/gj251290/airport-app`
**Equipo:** 3 integrantes
**Diseño:** Basado en mockups "The Horizon Flow" — paleta azul editorial, glassmorphism, tipografía Inter

---

## 1. Objetivo

Construir el frontend completo del sistema de reservas de vuelos consumiendo la API REST existente. El frontend debe ser una SPA (Single Page Application) compilada a WebAssembly con Dioxus, con manejo de autenticación JWT, CRUD visual para todas las entidades y un wizard de reservas guiado.

### Flujo funcional a demostrar

1. **Login** con JWT (`admin@admin.com` / `Admin123!`).
2. **Dashboard** con navegación a cada módulo.
3. **Catálogo:** Crear y listar aerolíneas, aeropuertos y vuelos.
4. **Usuarios:** Crear y listar usuarios.
5. **Reserva guiada (wizard):**
   - Seleccionar usuario.
   - Crear reserva en estado `HOLD`.
   - Agregar vuelos (ida / ida y vuelta) con selección visual.
   - Agregar pasajeros.
   - Confirmar reserva → estado `CONFIRMED`.
   - Ver total calculado por el backend.

## 2. Sistema de Diseño — The Horizon Flow

### Paleta de Colores (extraída de mockups)

```css
:root {
  /* Primary — Azul autoritario */
  --primary: #00429d;
  --primary-container: #0a58ca;
  --primary-fixed: #d9e2ff;
  --primary-fixed-dim: #b0c6ff;
  --on-primary: #ffffff;
  --on-primary-container: #ccd8ff;
  --on-primary-fixed: #001945;
  --on-primary-fixed-variant: #00419d;
  --inverse-primary: #b0c6ff;
  --surface-tint: #0958ca;

  /* Secondary */
  --secondary: #4c5d8a;
  --secondary-container: #b9cbfe;
  --secondary-fixed: #d9e2ff;
  --secondary-fixed-dim: #b4c6f8;
  --on-secondary: #ffffff;
  --on-secondary-container: #435581;
  --on-secondary-fixed: #031942;
  --on-secondary-fixed-variant: #344670;

  /* Tertiary — Naranja para urgencias */
  --tertiary: #7f2b00;
  --tertiary-container: #a63b01;
  --tertiary-fixed: #ffdbce;
  --tertiary-fixed-dim: #ffb599;
  --on-tertiary: #ffffff;
  --on-tertiary-container: #ffcebc;
  --on-tertiary-fixed: #370e00;
  --on-tertiary-fixed-variant: #7f2b00;

  /* Error */
  --error: #ba1a1a;
  --error-container: #ffdad6;
  --on-error: #ffffff;
  --on-error-container: #93000a;

  /* Surface — Jerarquía de capas */
  --background: #f8f9ff;
  --on-background: #121c28;
  --surface: #f8f9ff;
  --on-surface: #121c28;
  --surface-variant: #d9e3f4;
  --on-surface-variant: #424653;
  --inverse-surface: #27313e;
  --inverse-on-surface: #eaf1ff;
  --surface-dim: #d1dbec;
  --surface-bright: #f8f9ff;
  --surface-container-lowest: #ffffff;
  --surface-container-low: #eef4ff;
  --surface-container: #e5eeff;
  --surface-container-high: #dfe9fa;
  --surface-container-highest: #d9e3f4;

  /* Outline */
  --outline: #737785;
  --outline-variant: #c3c6d6;

  /* Estados de vuelo */
  --status-on-time: #059669;
  --status-on-time-bg: #d1fae5;
  --status-delayed: #d97706;
  --status-delayed-bg: #fef3c7;
  --status-boarding: #2563eb;
  --status-boarding-bg: #dbeafe;
  --status-cancelled: #dc2626;
  --status-cancelled-bg: #fee2e2;
  --status-confirmed: #059669;
  --status-confirmed-bg: #d1fae5;
  --status-booked: #2563eb;
  --status-booked-bg: #dbeafe;

  /* Sombras */
  --shadow-sm: 0 2px 4px rgba(18, 28, 40, 0.04);
  --shadow-md: 0 4px 12px rgba(18, 28, 40, 0.06);
  --shadow-lg: 0 10px 15px rgba(18, 28, 40, 0.08);
  --shadow-xl: 0 20px 25px rgba(18, 28, 40, 0.1);
  --shadow-primary: 0 4px 14px rgba(0, 66, 157, 0.25);

  /* Tipografía */
  --font-family: "Inter", -apple-system, BlinkMacSystemFont, sans-serif;

  /* Border Radius */
  --radius-sm: 0.25rem;
  --radius-md: 0.5rem;
  --radius-lg: 0.75rem;
  --radius-xl: 1rem;
  --radius-full: 9999px;

  /* Spacing */
  --space-xs: 0.25rem;
  --space-sm: 0.5rem;
  --space-md: 1rem;
  --space-lg: 1.5rem;
  --space-xl: 2rem;
  --space-2xl: 3rem;
  --space-3xl: 4rem;
}
```

### Reglas de Diseño

- **"No-Line" Rule:** No usar bordes de 1px para separar contenido. Usar cambios de color de superficie.
- _Glassmorphism:_ Navbar y overlays con `backdrop-filter: blur(20px)` y opacidad 80%.
- **Gradientes:** CTAs principales con gradiente `primary → primary-container` a 135°.
- **Labels:** Siempre uppercase, weight 500, letter-spacing 0.05em (estilo señalización aeropuerto).
- **Ghost Borders:** Si se necesita borde, usar `outline-variant` al 15% de opacidad.
- **Tonal Layering:** Elevar cards mediante delta de luminancia, no sombras.

## 3. Estructura del Proyecto

- **Convención de nombres:** Todos los archivos `.rs` y `.css` usan **una sola palabra** (sin guiones bajos ni concatenaciones).
- **Assets en Dioxus 0.7:** Todos los archivos estáticos (CSS, imágenes, fuentes) residen en la carpeta assets/ en la raíz del proyecto. Se referencian con la macro `asset!()` y se cargan con `document::Stylesheet` o `document::Link`. La macro genera un nombre con hash de contenido para cache busting automático .
- **Carpeta public/:** Solo para archivos que no son referenciados por código (ej. `robots.txt`, `.well-known`, favicons). `dx` los copia tal cual al bundle de salida .

```text
airport-app-frontend/
├── Cargo.toml [Gerardo]
├── Dioxus.toml [Nicolás]
├── .env.example [Nicolás]
├── .gitignore
├── README.md [Gerardo]
│
├── assets/                    ← Todos los estilos e imágenes
│   ├── main.css               ← Estilos globales + import de variables [Gerardo]
│   ├── variables.css          ← Paleta, spacing, tipografía, sombras [Gerardo]
│   │
│   ├── components/            ← Estilos por componente
│   │   ├── layout.css [Gerardo]
│   │   ├── navbar.css [Gerardo]
│   │   ├── sidebar.css [Gerardo]
│   │   ├── table.css [Gerardo]
│   │   ├── form.css [Gerardo]
│   │   ├── modal.css [Gerardo]
│   │   ├── card.css [Gerardo]
│   │   ├── timeline.css [Gerardo]
│   │   ├── search.css [Gerardo]
│   │   ├── filter.css [Gerardo]
│   │   ├── stepper.css [Gerardo]
│   │   ├── stats.css [Gerardo]
│   │   ├── summary.css [Gerardo]
│   │   ├── passengerform.css [Gerardo]
│   │   ├── loader.css [Gerardo]
│   │   └── alert.css [Gerardo]
│   │
│   └── pages/             ← Estilos por página
│       ├── login.css [Ángel]
│       ├── landing.css [Gerardo]
│       ├── dashboard.css [Gerardo]
│       ├── users.css [Ángel]
│       ├── airlines.css [Nicolás]
│       ├── airports.css [Nicolás]
│       ├── flights.css [Ángel]
│       ├── results.css [Nicolás]
│       ├── reservations.css [Gerardo]
│       ├── wizard.css [Gerardo]
│       ├── confirm.css [Gerardo]
│       ├── trips.css [Gerardo]
│       └── passengers.css [Gerardo]
│
├── public/               ← Archivos estáticos no referenciados por código
│   └── favicon.ico
│
└── src/
    ├── main.rs [Gerardo]
    ├── app.rs [Gerardo]
    ├── constants.rs [Ángel]
    │
    ├── state/
    │   ├── mod.rs [Gerardo]
    │   ├── auth.rs [Ángel]
    │   └── app.rs [Gerardo]
    │
    ├── api/
    │   ├── mod.rs [Gerardo]
    │   ├── client.rs [Gerardo]
    │   ├── auth.rs [Ángel]
    │   ├── users.rs [Ángel]
    │   ├── airlines.rs [Nicolás]
    │   ├── airports.rs [Nicolás]
    │   ├── flights.rs [Ángel]
    │   ├── reservations.rs [Gerardo]
    │   ├── passengers.rs [Gerardo]
    │   └── reservationflights.rs [Gerardo]
    │
    ├── models/
    │   ├── mod.rs [Gerardo]
    │   ├── auth.rs [Ángel]
    │   ├── user.rs [Ángel]
    │   ├── airline.rs [Nicolás]
    │   ├── airport.rs [Nicolás]
    │   ├── flight.rs [Ángel]
    │   ├── reservation.rs [Gerardo]
    │   ├── passenger.rs [Gerardo]
    │   └── reservationflight.rs [Gerardo]
    │
    ├── components/            ← Solo archivos .rs (sin CSS aquí)
    │   ├── mod.rs [Gerardo]
    │   ├── layout.rs [Gerardo]
    │   ├── navbar.rs [Gerardo]
    │   ├── sidebar.rs [Gerardo]
    │   ├── guard.rs [Ángel]
    │   ├── table.rs [Gerardo]
    │   ├── form.rs [Gerardo]
    │   ├── modal.rs [Gerardo]
    │   ├── card.rs [Gerardo]
    │   ├── timeline.rs [Gerardo]
    │   ├── search.rs [Gerardo]
    │   ├── filter.rs [Gerardo]
    │   ├── stepper.rs [Gerardo]
    │   ├── stats.rs [Gerardo]
    │   ├── summary.rs [Gerardo]
    │   ├── passengerform.rs [Gerardo]
    │   ├── loader.rs [Gerardo]
    │   └── alert.rs [Gerardo]
    │
    └── pages/                 ← Solo archivos .rs (sin CSS aquí)
        ├── mod.rs [Gerardo]
        ├── login.rs [Ángel]
        ├── landing.rs [Gerardo]
        ├── dashboard.rs [Gerardo]
        │
        ├── users/
        │   ├── mod.rs [Ángel]
        │   ├── list.rs [Ángel]
        │   ├── create.rs [Ángel]
        │   └── edit.rs [Ángel]
        │
        ├── airlines/
        │   ├── mod.rs [Nicolás]
        │   ├── list.rs [Nicolás]
        │   ├── create.rs [Nicolás]
        │   └── edit.rs [Nicolás]
        │
        ├── airports/
        │   ├── mod.rs [Nicolás]
        │   ├── list.rs [Nicolás]
        │   ├── create.rs [Nicolás]
        │   └── edit.rs [Nicolás]
        │
        ├── flights/
        │   ├── mod.rs [Ángel]
        │   ├── list.rs [Ángel]
        │   ├── create.rs [Ángel]
        │   ├── edit.rs [Ángel]
        │   └── results.rs [Nicolás]
        │
        ├── reservations/
        │   ├── mod.rs [Gerardo]
        │   ├── list.rs [Gerardo]
        │   ├── trips.rs [Gerardo]
        │   ├── create.rs [Gerardo]
        │   ├── edit.rs [Gerardo]
        │   ├── wizard.rs [Gerardo]
        │   └── confirm.rs [Gerardo]
        │
        └── passengers/
            ├── mod.rs [Gerardo]
            ├── list.rs [Gerardo]
            ├── create.rs [Gerardo]
            └── edit.rs [Gerardo]
```

## 4. Carga de Assets en Dioxus 0.7

**Patrón para incluir CSS**

Cada componente o página que necesite estilos propios debe declarar su asset y cargarlo con document::Stylesheet

```rust
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    const NAVBAR_CSS: Asset = asset!("/assets/components/navbar.css");

    rsx! {
        document::Stylesheet { href: NAVBAR_CSS }
        nav { class: "navbar", ... }
    }
}
```

**Patrón para estilos globales**

Los estilos globales (main.css, variables.css) se cargan una sola vez en el componente raíz (app.rs o layout.rs):

```rust
const GLOBAL_CSS: Asset = asset!("/assets/main.css");

rsx! {
    document::Stylesheet { href: GLOBAL_CSS }
    Router::<Route> {}
}
```

**Reglas de assets**

- **No usar include_str!()** — asset!() no incluye el contenido en el binario, sino que genera un path único con hash para carga en runtime.
- **Hot-reload automático:** Todos los archivos en assets/ se recargan en caliente al editarlos durante dx serve .
- **Optimización automática:** CSS se minifica automáticamente al hacer build.
- **SCSS soportado:** Si se prefiere, se puede usar .scss y se compila automáticamente.

## 5. Contrato con el Backend

| Recurso            | Endpoint                   | Métodos                                             |
| ------------------ | -------------------------- | --------------------------------------------------- |
| Auth               | `/api/auth/login`          | POST (form-data: username, password)                |
| Users              | `/api/users`               | GET, POST, PUT, DELETE                              |
| Airlines           | `/api/airlines`            | GET, POST, PUT, DELETE                              |
| Airports           | `/api/airports`            | GET, POST, PUT, DELETE                              |
| Flights            | `/api/flights`             | GET, POST, PUT, DELETE                              |
| Reservations       | `/api/reservations`        | GET, POST, PUT, DELETE                              |
| Passengers         | `/api/passengers`          | GET, POST, PUT, DELETE (filtro: `?reservation_id=`) |
| ReservationFlights | `/api/reservation-flights` | GET, POST, DELETE (filtro: `?reservation_id=`)      |

**Autenticación:** Bearer token en header Authorization.
**CORS:** El backend ya está configurado para http://localhost:8080 (Dioxus dev server).

## 6. División de Responsabilidades por Integrante

### Gerardo Andrés Jiménez Piedrahíta — Integrador + Motor de Reservas + Admin Dashboard

**Rol:** Arquitecto del frontend, integración final, flujo de negocio complejo, vistas admin.

**Archivos asignados:**

```text
Cargo.toml
README.md
src/main.rs
src/app.rs
src/state/mod.rs
src/state/app.rs
src/api/mod.rs
src/api/client.rs
src/api/reservations.rs
src/api/passengers.rs
src/api/reservationflights.rs
src/models/mod.rs
src/models/reservation.rs
src/models/passenger.rs
src/models/reservationflight.rs
src/components/mod.rs
src/components/layout.rs
src/components/sidebar.rs
src/components/timeline.rs
src/components/stepper.rs
src/components/stats.rs
src/components/summary.rs
src/components/passengerform.rs
src/pages/mod.rs
src/pages/dashboard.rs
src/pages/reservations/mod.rs
src/pages/reservations/list.rs
src/pages/reservations/create.rs
src/pages/reservations/edit.rs
src/pages/reservations/wizard.rs
src/pages/reservations/confirm.rs
src/pages/reservations/trips.rs
src/pages/passengers/mod.rs
src/pages/passengers/list.rs
src/pages/passengers/create.rs
src/pages/passengers/edit.rs
assets/components/layout.css
assets/components/sidebar.css
assets/components/timeline.css
assets/components/stepper.css
assets/components/stats.css
assets/components/summary.css
assets/components/passengerform.css
assets/pages/dashboard.css
assets/pages/reservations.css
assets/pages/wizard.css
assets/pages/confirm.css
assets/pages/trips.css
assets/pages/passengers.css
```

**Responsabilidades:**

- Configurar el proyecto base: `Cargo.toml`, estructura inicial.
- Implementar el router principal (`app.rs`) con rutas protegidas y layout dual (admin vs público).
- Implementar el layout admin completo (sidebar + área de contenido).
- Crear el cliente HTTP base con manejo de errores, headers automáticos y refresh de token.
- Implementar el estado global de la aplicación.
- Desarrollar el **wizard de reservas** completo (flujo de negocio principal):
  - Paso 1: Seleccionar usuario existente.
  - Paso 2: Crear reserva en estado `HOLD`.
  - Paso 3: Wizard de vuelos (solo ida / ida y vuelta) con selección visual.
  - Paso 4: Agregar N pasajeros (formularios expandibles).
  - Paso 5: Confirmar reserva → `CONFIRMED`.
  - Mostrar total calculado por backend.
- **Dashboard admin** con stats cards, gráficos simplificados, tabla de upcoming departures.
- **My Trips** — listado de reservas del usuario con tabs (Upcoming/Past/Cancelled).
- **Booking Confirmation** — página de confirmación con itinerario, pasajeros, acciones.
- CRUD completo de `reservations`, `passengers`, `reservationflights`.
- Revisar PRs, resolver conflictos, merge a `dev`.
- Documentar en `README.md` cómo correr el frontend.

### Nicolás Josué Grijalba Huertas — Catálogo + UI Kit Base + Landing + Search

**Rol:** Especialista en componentes reutilizables, módulos de catálogo, vistas públicas.

**Archivos asignados:**

```text
Dioxus.toml
.env.example
assets/main.css
assets/variables.css
assets/components/navbar.css
assets/components/table.css
assets/components/modal.css
assets/components/card.css
assets/components/search.css
assets/components/filter.css
assets/pages/landing.css
assets/pages/airlines.css
assets/pages/airports.css
assets/pages/results.css
src/api/airlines.rs
src/api/airports.rs
src/models/airline.rs
src/models/airport.rs
src/components/navbar.rs
src/components/table.rs
src/components/modal.rs
src/components/card.rs
src/components/search.rs
src/components/filter.rs
src/pages/landing.rs
src/pages/airlines/mod.rs
src/pages/airlines/list.rs
src/pages/airlines/create.rs
src/pages/airlines/edit.rs
src/pages/airports/mod.rs
src/pages/airports/list.rs
src/pages/airports/create.rs
src/pages/airports/edit.rs
src/pages/flights/results.rs
```

**Responsabilidades:**

- Definir el **sistema de diseño CSS** (`assets/variables.css`): paleta completa, espaciado, tipografía, sombras, breakpoints responsive.
- Crear `assets/main.css` con estilos globales, reset y utilidades.
- Crear los **componentes reutilizables base**:
  - `TopNavbar`: Navbar público con glassmorphism, links activos, avatar usuario.
  - `Table`: tabla genérica estilo admin dashboard con headers uppercase, filas hover, badges de estado, acciones.
  - `Modal`: diálogo reutilizable con header gradiente, slots para body/footer.
  - `FlightCard`: card de vuelo con línea de tiempo, precio, badge "Selected", estados (Sold Out).
  - `SearchBar`: barra de búsqueda hero con inputs estilizados, iconos, tabs Round Trip/One Way.
  - `FilterPanel`: panel lateral con checkboxes custom, range slider, botones de tiempo.
- Implementar CRUD visual completo de **aerolíneas** y **aeropuertos**:
  - Listado con tabla, botón crear, acciones editar/eliminar.
  - Formularios de creación y edición con validación visual.
- **Landing Page**:
  - Hero con imagen de fondo, overlay gradiente, search bar flotante.
  - Sección "Trending Destinations" con cards de imagen.
  - Newsletter section.
- **Search Results**:
  - Layout con sidebar de filtros + grid de flight cards.
  - Estados: selected, sold out, empty state.
- Configurar `Dioxus.toml` y assets estáticos.
- Crear `.env.example` con las variables necesarias (`API_BASE_URL`).
- Asegurar que los componentes sean **puros CSS** (sin clases de frameworks).

### Ángel David Gutiérrez Ladino — Auth + Users + Flights Admin + Configuración

**Rol:** Especialista en autenticación, seguridad y módulos de usuarios/vuelos admin.

**Archivos asignados:**

```text
src/constants.rs
src/state/auth.rs
src/api/auth.rs
src/api/users.rs
src/api/flights.rs
src/models/auth.rs
src/models/user.rs
src/models/flight.rs
src/components/form.rs
src/components/guard.rs
src/components/loader.rs
src/components/alert.rs
src/pages/login.rs
src/pages/users/mod.rs
src/pages/users/list.rs
src/pages/users/create.rs
src/pages/users/edit.rs
src/pages/flights/mod.rs
src/pages/flights/list.rs
src/pages/flights/create.rs
src/pages/flights/edit.rs
assets/components/form.css
assets/components/loader.css
assets/components/alert.css
assets/pages/login.css
assets/pages/users.css
assets/pages/flights.css
```

**Responsabilidades:**

- Implementar el **sistema de autenticación JWT completo**:
  - Página de login con formulario estilizado.
  - Almacenar token en `localStorage`.
  - Leer token al iniciar la app y restaurar sesión.
  - Inyectar `Authorization: Bearer <token>` en cada request.
  - Cerrar sesión y limpiar estado.
- Implementar el **guard de rutas protegidas** (`ProtectedRoute`): redirigir a login si no hay token.
- CRUD visual completo de **usuarios** y **vuelos** (admin):
  - Listado con tabla, crear, editar, eliminar.
  - En vuelos: selects dinámicos para aerolínea y aeropuertos (consumir APIs de Nicolás).
  - Modal de "New Flight Schedule" con formulario completo.
- Crear componentes reutilizables:
  - `Form`: inputs estilizados con labels uppercase, estados focus, ghost borders.
  - `Loading`: spinner animado con CSS puro (`@keyframes`).
  - `ErrorMessage`: banner de notificación para errores de API o éxito.
- Definir `constants.rs` con URLs y configuración global.

## 7. Convenciones y Reglas de Trabajo

### Git y Ramas

- **Rama base:** `dev`
- **Cada integrante crea una sola rama feature** desde `dev`:
  - Gerardo: `feat/frontend-core-and-booking`
  - Nicolás: `feat/frontend-catalog-and-ui`
  - Ángel: `feat/frontend-auth-and-services`
- **Commits en español.**
- **Abrir PR hacia** `dev`. Gerardo (integrador) hace merge tras revisión y aprobación.

### Código

- **Idioma del código:** Inglés (structs, funciones, variables, archivos).
- **Idioma de la UI:** Español (textos visibles al usuario).
- **CSS:** Puro, sin frameworks. Usar variables CSS definidas en `assets/variables.css`.
- **Modelos:** Deben reflejar fielmente los schemas Pydantic del backend.
- **Manejo de errores:** Toda petición HTTP debe manejar `loading`, `error` y `success`.
- **Nomenclatura CSS local:** Usar prefijo de módulo para evitar colisiones.

```css
/* Ejemplo: table.css — Nicolás */
.airport-table {
}
.airport-table__header {
}
.airport-table--striped {
}

/* Ejemplo: wizard.css — Gerardo */
.booking-wizard {
}
.booking-wizard__step {
}
.booking-wizard--active {
}
```

### Dependencias (Cargo.toml) — Dioxus 0.7

```toml
[dependencies]
dioxus = { version = "0.7", features = ["web"] }
dioxus-router = "0.7"
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = ["Storage", "Window"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["serde", "v4"] }
dioxus-free-icons = { version = "0.10", features = ["lucide"] }
```

## 8. Notas de Integración

1. **Gerardo** debe crear primero la estructura base (`main.rs`, `app.rs`, `Cargo.toml`, `assets/base`) para que los demás puedan trabajar en paralelo.
2. **Nicolás** debe definir `assets/variables.css` y `assets/main.css` lo antes posible para que Ángel y Gerardo usen las mismas clases y variables.
3. **Ángel** debe entregar `state/auth.rs` y `api/auth.rs` temprano para que Gerardo pueda integrar el `ProtectedRoute`.
4. Los modelos (`src/models/`) deben mantenerse sincronizados con los schemas Pydantic del backend.
5. El cliente HTTP (`api/client.rs`) debe soportar:
   - Base URL configurable por `.env`.
   - Header `Authorization` automático si hay token.
   - Parseo de errores JSON del backend.
   - Timeout de 10 segundos.
6. **No usar inline styles** para estilos estáticos. Úsalos **solo** para valores dinámicos.
7. **Tipografía:** Importar Inter desde Google Fonts vía `document::Link` en el componente raíz o incluirla en `assets/`.
8. **Assets:** Todos los CSS e imágenes se cargan con `asset!()` y `document::Stylesheet`. No usar `include_str!()` ni paths relativos manuales.
9. **Public:** Usar la carpeta `public/` solo para archivos que no son importados por código (favicons, `robots.txt`, etc.).
