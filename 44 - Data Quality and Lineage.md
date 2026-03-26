---
title: Data Quality and Lineage
tags: [data, quality, lineage, governance]
---

# Data Quality and Lineage

Bad data creates fake confidence, and fake confidence is especially dangerous because it often looks like sophistication. Curves look cleaner. Metrics look more stable. Labels appear sharper. The team feels like it is making progress. Meanwhile the underlying data may have changed meaning through a normalization tweak, a missing interval, a silent schema drift, or an unnoticed transform version. This is why data quality and lineage are not administrative concerns. They are epistemic concerns. They determine what the system is justified in believing about its own evidence.

Lineage is the discipline of making transformation history traceable. A useful research result should be explainable in terms of the raw source, the capture method, the normalization version, the missing-data assumptions, the replay provenance, and the feature-generation logic that produced it. That may sound heavyweight, but the alternative is much heavier in the long run. Without lineage, iteration speed turns into confusion. The team may produce more experiments and more charts, yet become less certain about what any result actually means.

Data quality itself is not only about catching corrupt records. It is about preserving semantic continuity. Was the source feed complete? Did the exchange channel behavior change? Did the normalization layer reinterpret an event type? Did a feature pipeline begin dropping an edge case silently? In many real workflows, the most dangerous data problems are not the ones that crash loudly. They are the ones that remain numerically plausible while shifting the interpretation of the dataset under your feet.

This is why quality tracking should be explicit. Source exchange, channel, capture time, normalization version, feature version, and missing-interval awareness all matter because they let you recover the story of how a dataset came to mean what it means. Once you can tell that story clearly, research becomes far more robust. Without it, the team is often left with strong opinions attached to weak evidence.

The core lesson is that lineage protects the meaning of results. It allows the platform to move quickly without forgetting what its own conclusions depend on.

Related:

- [[41 - Data Collection and Storage]]
- [[45 - Analytics and Post-Trade Review]]
- [[90 - Source Notes]]
