## Design

There are three components in the project: `server`, `client` and `end`.

As the design, `client` is run on mobile devices like Android phones. `end`
is a program on SBCs like Raspberry PIs, and `server` is a bridge and a
central controller that makes up the communication between
`client` and `end`.

On the initial run of `end`, it will generate a random device UUID and store
it locally.

Diagram of the system is like this:

```
        End
         ↓
End -> Server <- Client
         ↑
        End
```

End devices are multiple, and should have a long connection with
the server, to prepare to receive the order from
the client in any time. Also, the ends should send heartbeats
periodically to the server, to ensure they're alive.

## Protocol