we want internal service A to talk with service B, instead of using library in application level

we use a proxy in physical layer

an a collection of proxies for different protocals would form a service mesh


Proxies in linkerd data plane implement the features by controlling the communication between the application services


Control plane in turn coordinates the behvior of thse proxies and allows the operator to control and mointer the mesh

Four Golden Singals for infering the health of a internal service using its extranal outputs from that services

![[Pasted image 20220811132954.png]]

mTLS

![[Pasted image 20220811133400.png]]

