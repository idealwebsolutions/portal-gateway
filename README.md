# Service Registry
etcd is used as a distributed registry

# Flow/Idea

  REQ         REP
Provider <-> Discovery (register service to etcd) (one-to-many)
               |
  RES         SUR
Provider <-> Health (pings every registered service / sharing etcd) (one-to-many)

---------------------------------------------------------------
(pgm or udp)  SUB         PUB
Provider <-> Router (routes every message to its destination)

  PSH        PULL
Provider -> Broker (one-to-many)

  PSH      PULL
Broker -> Router (one-to-many)



Every Provider pushes fans out messages (irc and matrix messages for example)
Every Provides takes in action creators to carry out a task

Providers can subscribe to publisher using a unique topic
