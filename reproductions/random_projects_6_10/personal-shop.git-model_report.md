# Model report for file:///tmp/top-repos-quality-repos-tsc38vid/personal-shop.git HEAD 1c6ee0d8f873ca15edf37c40658ec31a045cf2ee

### Dump

```json
{'created_at': '2021-08-21 11:34:21',
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
 'size': '13.3 kB',
 'tags': [],
 'uuid': '91085798-25e2-4ed9-a1ee-2f30a6b327a4',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-tsc38vid/personal-shop.git 1c6ee0d8f873ca15edf37c40658ec31a045cf2ee

# javascript
5 rules, avg.len. 3.6
## train
PPCR: 0.743900
### report
macro
{'f1-score': 0.8538640267161208,
 'precision': 0.952599064130321,
 'recall': 0.8141232380164884,
 'support': 3140}
micro
{'f1-score': 0.9136942675159234,
 'precision': 0.9136942675159235,
 'recall': 0.9136942675159235,
 'support': 3140}
weighted
{'f1-score': 0.8994035474992644,
 'precision': 0.9168596277741382,
 'recall': 0.9136942675159235,
 'support': 3140}
### report_full
macro
{'f1-score': 0.6962481303912723,
 'precision': 0.952599064130321,
 'recall': 0.6093743007305258,
 'support': 4221}
micro
{'f1-score': 0.7795136530362722,
 'precision': 0.9136942675159235,
 'recall': 0.67969675432362,
 'support': 4221}
weighted
{'f1-score': 0.7334410199261213,
 'precision': 0.9195558056601523,
 'recall': 0.67969675432362,
 'support': 4221}
## test
PPCR: 0.684034
### report
macro
{'f1-score': 0.4818103818103818,
 'precision': 0.7187018786572221,
 'recall': 0.5562713401621853,
 'support': 407}
micro
{'f1-score': 0.724815724815725,
 'precision': 0.7248157248157249,
 'recall': 0.7248157248157249,
 'support': 407}
weighted
{'f1-score': 0.6486478054045621,
 'precision': 0.7312575055106624,
 'recall': 0.7248157248157249,
 'support': 407}
### report_full
macro
{'f1-score': 0.41854192849927563,
 'precision': 0.7187018786572221,
 'recall': 0.4642464134448459,
 'support': 595}
micro
{'f1-score': 0.5888223552894213,
 'precision': 0.7248157248157249,
 'recall': 0.4957983193277311,
 'support': 595}
weighted
{'f1-score': 0.5262951825235949,
 'precision': 0.7276384406549808,
 'recall': 0.4957983193277311,
 'support': 595}
```

## javascript
### Summary
4 rules, avg.len. 3.0

| | |
|-|-|
|Min support|106|
|Max support|181|
|Min confidence|0.9503816962242126|
|Max confidence|0.9972375631332397|

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
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.997. Support: 181.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.length ≥ 3<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.989. Support: 136.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 131.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.986. Support: 106.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.0, "max_conf": 0.9972375631332397, "max_support": 181, "min_conf": 0.9503816962242126, "min_support": 106, "num_rules": 4}}
```
</details>
