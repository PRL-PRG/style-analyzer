# Model report for file:///tmp/top-repos-quality-repos-2wtpzdq7/diaporama.git HEAD 1aeab0bd934ec4c7f3abb823e45332b44bd68d9f

### Dump

```json
{'created_at': '2021-08-29 20:34:47',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.2 kB',
 'tags': [],
 'uuid': '9a4e05f5-20c2-4fe5-b6d7-a29413eaab52',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-2wtpzdq7/diaporama.git 1aeab0bd934ec4c7f3abb823e45332b44bd68d9f

# javascript
14 rules, avg.len. 4.6
## train
PPCR: 0.935380
### report
macro
{'f1-score': 0.7841344602870569,
 'precision': 0.8055399754373268,
 'recall': 0.7649115288028377,
 'support': 13317}
micro
{'f1-score': 0.9288127956747015,
 'precision': 0.9288127956747015,
 'recall': 0.9288127956747015,
 'support': 13317}
weighted
{'f1-score': 0.9252734918165513,
 'precision': 0.9226178631304095,
 'recall': 0.9288127956747015,
 'support': 13317}
### report_full
macro
{'f1-score': 0.7432181193773584,
 'precision': 0.8055399754373268,
 'recall': 0.694792638601455,
 'support': 14237}
micro
{'f1-score': 0.8978006822965812,
 'precision': 0.9288127956747015,
 'recall': 0.868792582707031,
 'support': 14237}
weighted
{'f1-score': 0.8886945541864001,
 'precision': 0.9136520149243008,
 'recall': 0.868792582707031,
 'support': 14237}
## test
PPCR: 0.932560
### report
macro
{'f1-score': 0.7888786147194635,
 'precision': 0.8098965014119787,
 'recall': 0.7699311926047301,
 'support': 3457}
micro
{'f1-score': 0.9311541799247903,
 'precision': 0.9311541799247903,
 'recall': 0.9311541799247903,
 'support': 3457}
weighted
{'f1-score': 0.9277998368674506,
 'precision': 0.925691703593628,
 'recall': 0.9311541799247903,
 'support': 3457}
### report_full
macro
{'f1-score': 0.7533841108935031,
 'precision': 0.8098965014119787,
 'recall': 0.7087403842531405,
 'support': 3707}
micro
{'f1-score': 0.8986599664991625,
 'precision': 0.9311541799247903,
 'recall': 0.8683571621257081,
 'support': 3707}
weighted
{'f1-score': 0.8911424752782572,
 'precision': 0.9198991103962173,
 'recall': 0.8683571621257081,
 'support': 3707}
```

## javascript
### Summary
11 rules, avg.len. 4.1

| | |
|-|-|
|Min support|100|
|Max support|2410|
|Min confidence|0.9385964870452881|
|Max confidence|0.9972222447395325|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'max_depth': 10,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 2410.` |
| 2 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.995. Support: 100.` |
| 3 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 514.` |
| 4 | `  -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.974. Support: 370.` |
| 5 | `  •••start_col ≥ 11<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.961. Support: 115.` |
| 6 | `  +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.967. Support: 351.` |
| 7 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 408.` |
| 8 | `  +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 399.` |
| 9 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.997. Support: 180.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = function<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 166.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 140.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.090909090909091, "max_conf": 0.9972222447395325, "max_support": 2410, "min_conf": 0.9385964870452881, "min_support": 100, "num_rules": 11}}
```
</details>
