# Model report for file:///tmp/top-repos-quality-repos-beafqjhu/angular-starter.git HEAD 610df4725415c9826df81eb36f68765013f0cdcb

### Dump

```json
{'created_at': '2021-08-30 07:27:26',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.11.0-31-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '15.6 kB',
 'tags': [],
 'uuid': '34106b1a-f178-4eef-bb7f-f5a2b3a612f1',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-beafqjhu/angular-starter.git 610df4725415c9826df81eb36f68765013f0cdcb

# javascript
9 rules, avg.len. 4.8
## train
PPCR: 0.554716
### report
macro
{'f1-score': 0.5203412321660249,
 'precision': 0.5201054021652028,
 'recall': 0.5211784778450076,
 'support': 2423}
micro
{'f1-score': 0.928188196450681,
 'precision': 0.928188196450681,
 'recall': 0.928188196450681,
 'support': 2423}
weighted
{'f1-score': 0.9216119014958812,
 'precision': 0.9162184758456918,
 'recall': 0.928188196450681,
 'support': 2423}
### report_full
macro
{'f1-score': 0.43538576784752925,
 'precision': 0.5201054021652028,
 'recall': 0.37892158795796566,
 'support': 4368}
micro
{'f1-score': 0.6623472242674128,
 'precision': 0.928188196450681,
 'recall': 0.5148809523809523,
 'support': 4368}
weighted
{'f1-score': 0.6220268367760284,
 'precision': 0.7934635241049757,
 'recall': 0.5148809523809523,
 'support': 4368}
## test
PPCR: 0.533953
### report
macro
{'f1-score': 0.5333048671650885,
 'precision': 0.5428074104817242,
 'recall': 0.5251306989562653,
 'support': 574}
micro
{'f1-score': 0.9355400696864111,
 'precision': 0.9355400696864111,
 'recall': 0.9355400696864111,
 'support': 574}
weighted
{'f1-score': 0.9331224501586298,
 'precision': 0.9319696621165485,
 'recall': 0.9355400696864111,
 'support': 574}
### report_full
macro
{'f1-score': 0.4452704878326137,
 'precision': 0.5428074104817242,
 'recall': 0.38072414518792375,
 'support': 1075}
micro
{'f1-score': 0.6513038204972711,
 'precision': 0.9355400696864111,
 'recall': 0.49953488372093025,
 'support': 1075}
weighted
{'f1-score': 0.6132882084632753,
 'precision': 0.8004507881628793,
 'recall': 0.49953488372093025,
 'support': 1075}
```

## javascript
### Summary
5 rules, avg.len. 3.2

| | |
|-|-|
|Min support|107|
|Max support|809|
|Min confidence|0.9326328635215759|
|Max confidence|0.997826099395752|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 230.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.997. Support: 147.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.986. Support: 107.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 809.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 171.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.2, "max_conf": 0.997826099395752, "max_support": 809, "min_conf": 0.9326328635215759, "min_support": 107, "num_rules": 5}}
```
</details>
