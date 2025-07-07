{{/*
Expand the name of the chart.
*/}}
{{- define "findag.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "findag.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "findag.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "findag.labels" -}}
helm.sh/chart: {{ include "findag.chart" . }}
{{ include "findag.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "findag.selectorLabels" -}}
app.kubernetes.io/name: {{ include "findag.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "findag.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "findag.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}

{{/*
Create the name of the secret for JWT
*/}}
{{- define "findag.secretName" -}}
{{- printf "%s-secrets" (include "findag.fullname" .) }}
{{- end }}

{{/*
Create the name of the config map for prometheus
*/}}
{{- define "findag.configMapName" -}}
{{- printf "%s-prometheus-config" (include "findag.fullname" .) }}
{{- end }}

{{/*
Create the name of the persistent volume claim
*/}}
{{- define "findag.pvcName" -}}
{{- printf "%s-data" (include "findag.fullname" .) }}
{{- end }} 