<script lang="ts">
  import Sidebar from "./Sidebar.svelte";
  import Dbcontent from "./Dbcontent.svelte";
  import {
    Popover,
    Col,
    Row,
    Navbar,
    NavbarToggler,
    NavbarBrand,
    Nav,
    NavItem,
    NavLink,
    Icon,
  } from "sveltestrap";

  let isOpen = false;
</script>

<svelte:head>
  <link
    rel="stylesheet"
    href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.5.0/font/bootstrap-icons.css"
  />
</svelte:head>

<main>
  <Navbar color="light" light expand="md">
    <NavbarBrand href="/">crudThing</NavbarBrand>
    <NavbarToggler on:click={() => (isOpen = !isOpen)} />

    <Nav class="ms-auto" navbar>
      <NavItem>
        <NavLink id="link"
          >About
          <Popover placement="bottom" target="link" title="About"
            >ToDo: Write about section
          </Popover>
        </NavLink>
      </NavItem>
      <NavItem>
        <NavLink href="https://github.com/mprantl" target="_blank"
          ><Icon name="github" /></NavLink
        >
      </NavItem>
    </Nav>
  </Navbar>

  <Row cols={2}>
    <Col>
      <Sidebar />
    </Col>

    <Col>
      <Dbcontent />
    </Col>
  </Row>
</main>
