# Model report for file:///tmp/top-repos-quality-repos-_43lu_bh/chai-as-promised.git HEAD 39bfd34988132e443d703c8547fef4e2fbfa7a21

### Dump

```json
{'created_at': '2021-08-30 02:18:24',
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
 'size': '16.0 kB',
 'tags': [],
 'uuid': 'f44a4610-2f2e-4412-8065-60d81ca3ae6d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-_43lu_bh/chai-as-promised.git 39bfd34988132e443d703c8547fef4e2fbfa7a21

# javascript
11 rules, avg.len. 4.4
## train
PPCR: 0.950902
### report
macro
{'f1-score': 0.6609074390785109,
 'precision': 0.6739931486736969,
 'recall': 0.6519151020237499,
 'support': 7960}
micro
{'f1-score': 0.9185929648241206,
 'precision': 0.9185929648241206,
 'recall': 0.9185929648241206,
 'support': 7960}
weighted
{'f1-score': 0.9079750714623381,
 'precision': 0.9027611845282817,
 'recall': 0.9185929648241206,
 'support': 7960}
### report_full
macro
{'f1-score': 0.649785270735587,
 'precision': 0.6739931486736969,
 'recall': 0.6334102970613624,
 'support': 8371}
micro
{'f1-score': 0.8954748637560468,
 'precision': 0.9185929648241206,
 'recall': 0.8734918169872178,
 'support': 8371}
weighted
{'f1-score': 0.8692536954998057,
 'precision': 0.8726826858857161,
 'recall': 0.8734918169872178,
 'support': 8371}
## test
PPCR: 0.954708
### report
macro
{'f1-score': 0.6981413604175001,
 'precision': 0.700686787965474,
 'recall': 0.6959023812641183,
 'support': 4279}
micro
{'f1-score': 0.9745267585884553,
 'precision': 0.9745267585884553,
 'recall': 0.9745267585884553,
 'support': 4279}
weighted
{'f1-score': 0.9683967041676518,
 'precision': 0.9624979000852737,
 'recall': 0.9745267585884553,
 'support': 4279}
### report_full
macro
{'f1-score': 0.6817557595706576,
 'precision': 0.700686787965474,
 'recall': 0.6665710052837327,
 'support': 4482}
micro
{'f1-score': 0.9519461248715901,
 'precision': 0.9745267585884553,
 'recall': 0.9303882195448461,
 'support': 4482}
weighted
{'f1-score': 0.9322182081192322,
 'precision': 0.9354751968528211,
 'recall': 0.9303882195448461,
 'support': 4482}
```

## javascript
### Summary
6 rules, avg.len. 3.5

| | |
|-|-|
|Min support|106|
|Max support|354|
|Min confidence|0.9952830076217651|
|Max confidence|0.9985875487327576|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.998. Support: 295.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.997. Support: 186.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.995. Support: 106.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.999. Support: 354.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.998. Support: 202.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 261.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.5, "max_conf": 0.9985875487327576, "max_support": 354, "min_conf": 0.9952830076217651, "min_support": 106, "num_rules": 6}}
```
</details>
