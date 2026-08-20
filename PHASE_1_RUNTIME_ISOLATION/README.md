# Phase 1 Runtime Isolation

This isolated workspace implements only `BC_RUNTIME_ISOLATION`. A separate local worker receives an untrusted workload request while its authority database and protected resource root are supplied through a trusted launcher boundary. Database-bound opaque capabilities, actor identity, currentness, domain membership, and canonical resource containment are checked before access.

The evidence is bounded local implementation evidence. It does not claim containers, VMs, kernel security, production multi-tenancy, recovery, audit certification, deployment authority, or independent certification.

