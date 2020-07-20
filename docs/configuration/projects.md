# Project Configuration

In `static` and `proxy` mode, you can configure project settings on the file system.

Static project configurations are found under the `projects` subdirectory of the Relay configuration directory, By default, this is located at `.relay/projects`.

To configure projects, add files named `<PROJECT_ID>.json` in that location:

```
.relay/
└── projects/
    ├── 17.json
    ├── 21.json
    └── 42.json
```

Project configurations are an extensible format, mostly consisting of optional
fields. The minimal configuration **must** contain the following fields:

```json
{
  "slug": "my-project",
  "publicKeys": [
    {
      "publicKey": "<DSN_KEY>",
      "isEnabled": true
    }
  ],
  "config": {
    "allowedDomains": ["*"]
  }
}
```

**Note:** The public key (`<DSN_KEY>`) is the key of the project's DSN and it
has nothing to do with the Relay public key that is used for Relay registration.

## Basic Options

### `slug`

```json
{
  "slug": "my-project"
}
```

The short name of the project, displayed in Sentry. This value is currently
required for Relay to accept events.

### `disabled`

```json
{
  "disabled": false
}
```

Whether the project is disabled. If set to `true`, the Relay will drop all
events sent to this project.

### `publicKeys`

```json
{
  "publicKeys": [
    {
      "publicKey": "12345abcdb1e4c123490ecec89c1f199",
      "isEnabled": true
    }
  ]
}
```

A list of known public keys (the public key in a DSN) and whether events using
that key should be accepted.

You can obtain the key by going into the _Sentry > Project Settings > Client
Keys (DSN)_ . The public key can be extracted from the DSN. In the DSN example
below:

```
https://12345abcdb1e4c123490ecec89c1f199@o1.ingest.sentry.io/2244
```

the key is:

```
12345abcdb1e4c123490ecec89c1f199
```

A project may contain multiple public keys, only messages using enabled project
keys will be processed. Likewise, keys can be disabled using the `isEnabled`
flag.

## `config.allowedDomains`

```json
{
  "config": {
    "allowedDomains": ["mycompany.com"]
  }
}
```

Configure _Origin_ or _Referer_ URLs which Sentry should accept events from.
This is corresponds to the _Allowed Domains_"_ setting in the Sentry UI.

**Warning**: An empty list rejects all origins. Use the default `["*"]` to allow
all origins.

## `config.piiConfig`

```json
{
  "config": {
    "piiConfig": {
      // ...
    }
  }
}
```

See _[PII Configuration]_.

[getting started]: ../../
[PII Configuration]: ../pii-config/index.md