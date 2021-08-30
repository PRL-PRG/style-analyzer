# Model report for file:///tmp/top-repos-quality-repos-ymn6j1l9/takana.git HEAD 3e274d5990392590ce44b00f1c0acd680f98cf7b

### Dump

```json
{'created_at': '2021-08-29 22:27:53',
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
 'size': '16.6 kB',
 'tags': [],
 'uuid': 'da437206-35e3-4a06-93a2-e4e2635fabbd',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ymn6j1l9/takana.git 3e274d5990392590ce44b00f1c0acd680f98cf7b

# javascript
11 rules, avg.len. 4.5
## train
PPCR: 0.862017
### report
macro
{'f1-score': 0.5637569366589202,
 'precision': 0.5636966153079602,
 'recall': 0.5659816680895697,
 'support': 11470}
micro
{'f1-score': 0.93487358326068,
 'precision': 0.93487358326068,
 'recall': 0.93487358326068,
 'support': 11470}
weighted
{'f1-score': 0.9256004073674816,
 'precision': 0.9182975545025951,
 'recall': 0.93487358326068,
 'support': 11470}
### report_full
macro
{'f1-score': 0.507228045936575,
 'precision': 0.5636966153079602,
 'recall': 0.48330469012065425,
 'support': 13306}
micro
{'f1-score': 0.8655957378107847,
 'precision': 0.93487358326068,
 'recall': 0.805877047948294,
 'support': 13306}
weighted
{'f1-score': 0.826990578247224,
 'precision': 0.8598113389133203,
 'recall': 0.805877047948294,
 'support': 13306}
## test
PPCR: 0.855286
### report
macro
{'f1-score': 0.5486232690956825,
 'precision': 0.5387314568809123,
 'recall': 0.5643889990011332,
 'support': 2500}
micro
{'f1-score': 0.9216, 'precision': 0.9216, 'recall': 0.9216, 'support': 2500}
weighted
{'f1-score': 0.9108064137803722,
 'precision': 0.9034115581895352,
 'recall': 0.9216,
 'support': 2500}
### report_full
macro
{'f1-score': 0.515774044478913,
 'precision': 0.5387314568809123,
 'recall': 0.4973560312280475,
 'support': 2923}
micro
{'f1-score': 0.8497141803429836,
 'precision': 0.9216,
 'recall': 0.7882312692439275,
 'support': 2923}
weighted
{'f1-score': 0.8061819110292135,
 'precision': 0.8323152240846919,
 'recall': 0.7882312692439275,
 'support': 2923}
```

## javascript
### Summary
5 rules, avg.len. 4.4

| | |
|-|-|
|Min support|109|
|Max support|2512|
|Min confidence|0.9220183491706848|
|Max confidence|0.998792290687561|

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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 2512.` |
| 2 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 414.` |
| 3 | `  -1.reserved = ;<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.922. Support: 109.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 1170.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 266.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.4, "max_conf": 0.998792290687561, "max_support": 2512, "min_conf": 0.9220183491706848, "min_support": 109, "num_rules": 5}}
```
</details>
