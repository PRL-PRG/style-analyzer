# Model report for file:///tmp/top-repos-quality-repos-xskfgb4j/npm-shrinkwrap.git HEAD 2c9016a1af6301943179ce1e552068fd60ae800c

### Dump

```json
{'created_at': '2021-08-29 10:42:55',
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
 'size': '16.3 kB',
 'tags': [],
 'uuid': '1629a427-6911-4374-8dda-bffd7c47c841',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-xskfgb4j/npm-shrinkwrap.git 2c9016a1af6301943179ce1e552068fd60ae800c

# javascript
14 rules, avg.len. 5.4
## train
PPCR: 0.918330
### report
macro
{'f1-score': 0.8034644804038951,
 'precision': 0.8119120384848065,
 'recall': 0.7999466956609785,
 'support': 7961}
micro
{'f1-score': 0.9433488255244316,
 'precision': 0.9433488255244316,
 'recall': 0.9433488255244316,
 'support': 7961}
weighted
{'f1-score': 0.9373764822463779,
 'precision': 0.9339725169002846,
 'recall': 0.9433488255244316,
 'support': 7961}
### report_full
macro
{'f1-score': 0.7308170691638901,
 'precision': 0.8119120384848065,
 'recall': 0.6845009664504522,
 'support': 8669}
micro
{'f1-score': 0.9031870114251352,
 'precision': 0.9433488255244316,
 'recall': 0.8663052255162071,
 'support': 8669}
weighted
{'f1-score': 0.8817225615722915,
 'precision': 0.9084894971030245,
 'recall': 0.8663052255162071,
 'support': 8669}
## test
PPCR: 0.902913
### report
macro
{'f1-score': 0.822378069299874,
 'precision': 0.8232437940588623,
 'recall': 0.8233367070547218,
 'support': 1674}
micro
{'f1-score': 0.9456391875746715,
 'precision': 0.9456391875746715,
 'recall': 0.9456391875746715,
 'support': 1674}
weighted
{'f1-score': 0.9371196250715063,
 'precision': 0.931007146434366,
 'recall': 0.9456391875746715,
 'support': 1674}
### report_full
macro
{'f1-score': 0.6956241814185267,
 'precision': 0.8232437940588623,
 'recall': 0.6687313155304299,
 'support': 1854}
micro
{'f1-score': 0.8973922902494331,
 'precision': 0.9456391875746715,
 'recall': 0.8538295577130529,
 'support': 1854}
weighted
{'f1-score': 0.8762149436388822,
 'precision': 0.9210671482043865,
 'recall': 0.8538295577130529,
 'support': 1854}
```

## javascript
### Summary
10 rules, avg.len. 4.4

| | |
|-|-|
|Min support|93|
|Max support|1672|
|Min confidence|0.9617646932601929|
|Max confidence|0.9979423880577087|

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
| 1 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.998. Support: 243.` |
| 2 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 1672.` |
| 3 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = {<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.983. Support: 263.` |
| 4 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {{}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 510.` |
| 5 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.995. Support: 93.` |
| 6 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 504.` |
| 7 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 389.` |
| 8 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 226.` |
| 9 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.988. Support: 210.` |
| 10 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.995. Support: 103.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.4, "max_conf": 0.9979423880577087, "max_support": 1672, "min_conf": 0.9617646932601929, "min_support": 93, "num_rules": 10}}
```
</details>
