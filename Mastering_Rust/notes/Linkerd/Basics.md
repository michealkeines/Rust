A service mesh incldude a

Data plane
Control plane

![[Pasted image 20220811102152.png]]

If a request is made from the private network out to a public IP, the public server/endpoint responds back to that request using a port number that was defined in the request, and firewall allows that connection since its aware of an initiated session based on that port number. See picture below for reference.

reponse back for a request made from inside is not igress request

![[Pasted image 20220811102556.png]]


let’s define Ingress. As you might be guessing by now, Ingress refers to unsolicited traffic sent from an address in public internet to the private network – it is not a response to a request initiated by an inside system. In this case, firewalls are designed to decline this request unless there are specific policy and configuration that allows ingress connections. See picture below for reference


![[Pasted image 20220811102648.png]]


the service mesh in kubernetes is typicallly implements as a set of network proxies deployed alongside a sidecar of application code, these proxies serve as an introduction point for service mesh features and manage communication between the microservices.

the data plane of the kuberneties service mesh is made up by the proxies, which the control plane controls

A pod in kubernetes is a wrapper for containers

a pod can contain any numberr of containers

![[Pasted image 20220811110545.png]]

test env

kubectl apply -f kube-deployment.yml

kubectl get svc

minikube service web

or minikube service web --url

delete pods

# delete all pods

kubectl delete --all pods --namespace=default

# deete all deployments

kubectl delete --all deployments --namespace=default

# delete all services

kubectl delete --all services --namespace=default

