package collector

import "github.com/prometheus/client_golang/prometheus"

func emitEnumStateMetric[T ~string](
	ch chan<- prometheus.Metric,
	desc *prometheus.Desc,
	currentState T,
	allStates []T,
	labelValues ...string,
) {
	for _, state := range allStates {
		value := 0.0
		if state == currentState {
			value = 1.0
		}

		labels := append(labelValues, string(state))
		ch <- prometheus.MustNewConstMetric(desc, prometheus.GaugeValue, value, labels...)
	}
}
