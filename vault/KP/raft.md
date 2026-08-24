---
id: "01J5K80123456789ABCDEFGHKP"
title: "Raft"
type: note
status: active
parents: [distributed-systems]
share_id: "sh_raft_99"
visibility: "collaborative"
---
# Raft Consensus Protocol

- Leader election phase. ^raft-elem-01
  - Heartbeat timeout triggers election. ^raft-elem-02
- Log replication phase. ^raft-log-01
- Transversal dependency on state machines: ((cs-root-01))
