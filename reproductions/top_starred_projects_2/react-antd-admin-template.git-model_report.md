# Model report for file:///tmp/top-repos-quality-repos-994dmf85/react-antd-admin-template.git HEAD 096a1d5bd8bb140b6fe74ab5905d2f3d86399308

### Dump

```json
{'created_at': '2021-08-30 04:32:11',
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
 'size': '16.3 kB',
 'tags': [],
 'uuid': 'd24e5a7b-8609-42af-872d-c5376a5029b9',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-994dmf85/react-antd-admin-template.git 096a1d5bd8bb140b6fe74ab5905d2f3d86399308

# javascript
8 rules, avg.len. 6.2
## train
PPCR: 0.454816
### report
macro
{'f1-score': 0.35718576979793437,
 'precision': 0.357439329657107,
 'recall': 0.357064271763201,
 'support': 3976}
micro
{'f1-score': 0.9655432595573441,
 'precision': 0.9655432595573441,
 'recall': 0.9655432595573441,
 'support': 3976}
weighted
{'f1-score': 0.954387028661179,
 'precision': 0.9436904286293355,
 'recall': 0.9655432595573441,
 'support': 3976}
### report_full
macro
{'f1-score': 0.1832594903297501,
 'precision': 0.357439329657107,
 'recall': 0.14794920139480763,
 'support': 8742}
micro
{'f1-score': 0.6037112753577607,
 'precision': 0.9655432595573441,
 'recall': 0.43914436055822464,
 'support': 8742}
weighted
{'f1-score': 0.503830367024466,
 'precision': 0.7743367549629205,
 'recall': 0.43914436055822464,
 'support': 8742}
## test
PPCR: 0.477798
### report
macro
{'f1-score': 0.2681467783084005,
 'precision': 0.26860693772458477,
 'recall': 0.2715714925031629,
 'support': 1076}
micro
{'f1-score': 0.921003717472119,
 'precision': 0.921003717472119,
 'recall': 0.921003717472119,
 'support': 1076}
weighted
{'f1-score': 0.9030431944429991,
 'precision': 0.889197284561378,
 'recall': 0.921003717472119,
 'support': 1076}
### report_full
macro
{'f1-score': 0.15813883976883816,
 'precision': 0.26860693772458477,
 'recall': 0.13419173108770424,
 'support': 2252}
micro
{'f1-score': 0.5955528846153846,
 'precision': 0.921003717472119,
 'recall': 0.4400532859680284,
 'support': 2252}
weighted
{'f1-score': 0.49901970043211086,
 'precision': 0.7351099292902658,
 'recall': 0.4400532859680284,
 'support': 2252}
```

## javascript
### Summary
4 rules, avg.len. 5.5

| | |
|-|-|
|Min support|139|
|Max support|1049|
|Min confidence|0.9827806353569031|
|Max confidence|0.9964028596878052|

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
                     'min_samples_split': 190,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 784.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 249.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 139.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {BINARY, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 1049.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.5, "max_conf": 0.9964028596878052, "max_support": 1049, "min_conf": 0.9827806353569031, "min_support": 139, "num_rules": 4}}
```
</details>
