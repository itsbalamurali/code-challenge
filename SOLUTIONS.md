## K8s Debugging

Noticed Issues:

- Pod command/entrypoint is `sleep`.
- Service target port mapping is incorrect.
- Select label mapping is incorrect.

Fix manifests:

`deployment.yaml`

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
    name: nginx
spec:
    selector:
        matchLabels:
            app: nginx
        replicas: 2
template:
    metadata:
        labels:
            app: nginx
    spec:
        containers:
            - name: nginx
              image: nginx:1.14.2 #removed sleep command override.
              ports:
                  - containerPort: 80
```

`service.yaml`

```yaml
apiVersion: v1
kind: Service
metadata:
    name: nginx
spec:
    type: NodePort
    selector:
        app: nginx
    ports:
        - port: 80 #mapped to 80 for convenience
          targetPort: 80 #81 was wrong where the actual container port in the deployment is 80.
          nodePort: 30007
```

## GCP Architecture

I would push the existing docker images to **Artifactory** and use **Cloud Run** to run the python app.

Reasons:

-   Easy to Deploy NO maintenance.
-   Pay per use without dedicated commitments.
-   Auto scaling.
-   Integrates well with other google products.
