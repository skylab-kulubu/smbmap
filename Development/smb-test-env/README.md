# SMB Test Environment Setup

## 4. Starting the Container and Domain Provisioning

To start the container, run the following command:

```bash
docker-compose up -d
```

On the first startup, the Active Directory (AD) database will be created automatically. If it is not created automatically, you can create it manually with the following command:

```bash
docker exec -it samba-ad samba-tool domain provision \
  --domain=SKYSEC \
  --realm=SKYSEC.LOCAL \
  --adminpass=Passw0rd! \
  --dns-backend=SAMBA_INTERNAL
```

To restart the service:

```bash
docker restart samba-ad
```

---

## 5. User Management

### a) From the Command Line

#### Adding a New User

To add a new user:

```bash
docker exec -it samba-ad samba-tool user create ali P4ssw0rd! --must-change-at-next-login
```

#### Updating User Information

To update a user's account expiration date:

```bash
docker exec -it samba-ad samba-tool user setexpiry ali --account-expires=2025-12-31
```

#### Adding a User to a Group

To add a user to a group:

```bash
docker exec -it samba-ad samba-tool group addmembers "Domain Users" ali
```

---

### b) Windows Remote Administration Tools (RSAT)

To manage using the Active Directory Users and Computers (ADUC) console on Windows:

1. Open the ADUC console on a host or another Windows machine.
2. Connect to the LDAP server by specifying your Docker host and port `389`:
   - **Server:** `<docker-host-ip>:389`
   - **Domain:** `EXAMPLE.LOCAL`
   - **Credentials:** `administrator / Passw0rd!`

You can now manage users, groups, OUs, etc., through the Windows GUI.

---

## 6. LDAPS (Optional)

If you want to secure your traffic using encrypted LDAP (LDAPS):

1. Place the certificate files in the following locations:
   - `/var/lib/samba/private/samba_cert.pem`
   - `/var/lib/samba/private/samba_key.pem`

2. Add the following lines to the `smb.conf` file:

   ```ini
   tls enabled  = yes
   tls keyfile  = /var/lib/samba/private/samba_key.pem
   tls certfile = /var/lib/samba/private/samba_cert.pem
   ```

3. Restart the container:

   ```bash
   docker restart samba-ad
   ```
