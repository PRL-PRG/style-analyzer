# Model report for file:///tmp/top-repos-quality-repos-t71h_dym/objc.git HEAD 14168534948813a7e1b5e656c879b1ed032f3306

### Dump

```json
{'created_at': '2021-08-25 03:49:55',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.1 kB',
 'tags': [],
 'uuid': '502a3183-919b-4f65-a378-d6f856133ca6',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-t71h_dym/objc.git 14168534948813a7e1b5e656c879b1ed032f3306

# javascript
12 rules, avg.len. 5.1
## train
PPCR: 0.885288
### report
macro
{'f1-score': 0.7437910144637814,
 'precision': 0.7793702110325925,
 'recall': 0.7190990326291882,
 'support': 9369}
micro
{'f1-score': 0.9000960614793467,
 'precision': 0.9000960614793467,
 'recall': 0.9000960614793467,
 'support': 9369}
weighted
{'f1-score': 0.8944137619345813,
 'precision': 0.8926292469087015,
 'recall': 0.9000960614793467,
 'support': 9369}
### report_full
macro
{'f1-score': 0.6812981769195645,
 'precision': 0.7793702110325925,
 'recall': 0.6386746283463601,
 'support': 10583}
micro
{'f1-score': 0.8453287890938253,
 'precision': 0.9000960614793467,
 'recall': 0.7968439950864594,
 'support': 10583}
weighted
{'f1-score': 0.8166097330544904,
 'precision': 0.8611703643511655,
 'recall': 0.7968439950864594,
 'support': 10583}
## test
PPCR: 0.895802
### report
macro
{'f1-score': 0.6918350322485817,
 'precision': 0.7373911485901229,
 'recall': 0.6673990882354787,
 'support': 2347}
micro
{'f1-score': 0.8466126970600767,
 'precision': 0.8466126970600767,
 'recall': 0.8466126970600767,
 'support': 2347}
weighted
{'f1-score': 0.8382714274154137,
 'precision': 0.8375411619201543,
 'recall': 0.8466126970600767,
 'support': 2347}
### report_full
macro
{'f1-score': 0.6501053561100276,
 'precision': 0.7373911485901229,
 'recall': 0.6158120220941352,
 'support': 2620}
micro
{'f1-score': 0.8000805315079526,
 'precision': 0.8466126970600767,
 'recall': 0.7583969465648855,
 'support': 2620}
weighted
{'f1-score': 0.7735351729272779,
 'precision': 0.8092635093647847,
 'recall': 0.7583969465648855,
 'support': 2620}
```

## javascript
### Summary
5 rules, avg.len. 4.6

| | |
|-|-|
|Min support|91|
|Max support|2684|
|Min confidence|0.9625558853149414|
|Max confidence|0.9988763928413391|

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
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 445.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 91.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.991. Support: 174.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 200.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 2684.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.6, "max_conf": 0.9988763928413391, "max_support": 2684, "min_conf": 0.9625558853149414, "min_support": 91, "num_rules": 5}}
```
</details>
