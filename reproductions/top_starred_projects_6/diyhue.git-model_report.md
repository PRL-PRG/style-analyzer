# Model report for file:///tmp/top-repos-quality-repos-juvoghh_/diyhue.git HEAD d386e072bec57decaf14c76ba53b2e4caf8fa4f1

### Dump

```json
{'created_at': '2021-08-29 21:44:52',
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
 'size': '14.6 kB',
 'tags': [],
 'uuid': '1034c189-b14d-4313-884f-23fbe54373c4',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-juvoghh_/diyhue.git d386e072bec57decaf14c76ba53b2e4caf8fa4f1

# javascript
6 rules, avg.len. 8.2
## train
PPCR: 0.661530
### report
macro
{'f1-score': 0.45773894113653757,
 'precision': 0.47655775706110676,
 'recall': 0.446366585830557,
 'support': 5795}
micro
{'f1-score': 0.9211389128559103,
 'precision': 0.9211389128559103,
 'recall': 0.9211389128559103,
 'support': 5795}
weighted
{'f1-score': 0.9008911897885273,
 'precision': 0.8885573222709597,
 'recall': 0.9211389128559103,
 'support': 5795}
### report_full
macro
{'f1-score': 0.32101414403666634,
 'precision': 0.47655775706110676,
 'recall': 0.2698543745478546,
 'support': 8760}
micro
{'f1-score': 0.733493644795603,
 'precision': 0.9211389128559103,
 'recall': 0.6093607305936073,
 'support': 8760}
weighted
{'f1-score': 0.6507772055646854,
 'precision': 0.8054896578653389,
 'recall': 0.6093607305936073,
 'support': 8760}
## test
PPCR: 0.416568
### report
macro
{'f1-score': 0.26310641627543035,
 'precision': 0.2904662248575747,
 'recall': 0.25273940735749467,
 'support': 352}
micro
{'f1-score': 0.8551136363636364,
 'precision': 0.8551136363636364,
 'recall': 0.8551136363636364,
 'support': 352}
weighted
{'f1-score': 0.8310092562953478,
 'precision': 0.8350288946317025,
 'recall': 0.8551136363636364,
 'support': 352}
### report_full
macro
{'f1-score': 0.16973445579783553,
 'precision': 0.2904662248575747,
 'recall': 0.15808749749147102,
 'support': 845}
micro
{'f1-score': 0.5029239766081872,
 'precision': 0.8551136363636364,
 'recall': 0.3562130177514793,
 'support': 845}
weighted
{'f1-score': 0.3952231349507449,
 'precision': 0.7920751749658942,
 'recall': 0.3562130177514793,
 'support': 845}
```

## javascript
### Summary
4 rules, avg.len. 5.8

| | |
|-|-|
|Min support|146|
|Max support|275|
|Min confidence|0.9327272772789001|
|Max confidence|0.9975728392601013|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.998. Support: 206.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.933. Support: 275.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 146.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclaration<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 155.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.75, "max_conf": 0.9975728392601013, "max_support": 275, "min_conf": 0.9327272772789001, "min_support": 146, "num_rules": 4}}
```
</details>
