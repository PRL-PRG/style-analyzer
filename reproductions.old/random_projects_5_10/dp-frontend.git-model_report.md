# Model report for file:///tmp/top-repos-quality-repos-5_4o86vv/dp-frontend.git HEAD e4ff586fe9626b3a27a1dbb8c5c60921a81537e0

### Dump

```json
{'created_at': '2021-08-21 18:49:38',
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
 'size': '15.4 kB',
 'tags': [],
 'uuid': '0bf06cee-a089-4300-ab4c-46dd07cc473f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-5_4o86vv/dp-frontend.git e4ff586fe9626b3a27a1dbb8c5c60921a81537e0

# javascript
7 rules, avg.len. 4.4
## train
PPCR: 0.633398
### report
macro
{'f1-score': 0.4055650284833997,
 'precision': 0.4035102279826909,
 'recall': 0.4091850172008019,
 'support': 2951}
micro
{'f1-score': 0.9254490003388682,
 'precision': 0.9254490003388682,
 'recall': 0.9254490003388682,
 'support': 2951}
weighted
{'f1-score': 0.9072385260861933,
 'precision': 0.8919555313005787,
 'recall': 0.9254490003388682,
 'support': 2951}
### report_full
macro
{'f1-score': 0.353426868118749,
 'precision': 0.4035102279826909,
 'recall': 0.31549928188895743,
 'support': 4659}
micro
{'f1-score': 0.7177398160315375,
 'precision': 0.9254490003388682,
 'recall': 0.5861772912642198,
 'support': 4659}
weighted
{'f1-score': 0.672531455218784,
 'precision': 0.7910161044147653,
 'recall': 0.5861772912642198,
 'support': 4659}
## test
PPCR: 0.701389
### report
macro
{'f1-score': 0.39520134303773896,
 'precision': 0.39385524701683955,
 'recall': 0.4019107643057223,
 'support': 606}
micro
{'f1-score': 0.905940594059406,
 'precision': 0.905940594059406,
 'recall': 0.905940594059406,
 'support': 606}
weighted
{'f1-score': 0.8725907168863498,
 'precision': 0.8464426770545906,
 'recall': 0.905940594059406,
 'support': 606}
### report_full
macro
{'f1-score': 0.35781157092241916,
 'precision': 0.39385524701683955,
 'recall': 0.3281572575506563,
 'support': 864}
micro
{'f1-score': 0.746938775510204,
 'precision': 0.905940594059406,
 'recall': 0.6354166666666666,
 'support': 864}
weighted
{'f1-score': 0.6881741384718512,
 'precision': 0.7509786914158498,
 'recall': 0.6354166666666666,
 'support': 864}
```

## javascript
### Summary
5 rules, avg.len. 3.6

| | |
|-|-|
|Min support|109|
|Max support|564|
|Min confidence|0.9476386308670044|
|Max confidence|0.9961538314819336|

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
                     'min_samples_leaf': 97,
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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 487.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.996. Support: 130.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.995. Support: 109.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 213.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 564.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.6, "max_conf": 0.9961538314819336, "max_support": 564, "min_conf": 0.9476386308670044, "min_support": 109, "num_rules": 5}}
```
</details>
