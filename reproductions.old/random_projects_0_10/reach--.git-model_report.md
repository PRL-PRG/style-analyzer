# Model report for file:///tmp/top-repos-quality-repos-fvt_uisk/reach--.git HEAD 9fdd933ae18aabcb7d8b61e398829b5f63e657a0

### Dump

```json
{'created_at': '2021-08-22 18:54:45',
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
 'size': '16.8 kB',
 'tags': [],
 'uuid': '290f531c-3993-4c32-98de-2bce63353cab',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-fvt_uisk/reach--.git 9fdd933ae18aabcb7d8b61e398829b5f63e657a0

# javascript
15 rules, avg.len. 5.7
## train
PPCR: 0.901961
### report
macro
{'f1-score': 0.5716606448047765,
 'precision': 0.5848937136032095,
 'recall': 0.5621390817109828,
 'support': 13892}
micro
{'f1-score': 0.9429887705154045,
 'precision': 0.9429887705154045,
 'recall': 0.9429887705154045,
 'support': 13892}
weighted
{'f1-score': 0.9336162330202123,
 'precision': 0.9256396600120294,
 'recall': 0.9429887705154045,
 'support': 13892}
### report_full
macro
{'f1-score': 0.5156151217158378,
 'precision': 0.5848937136032095,
 'recall': 0.48597754763269185,
 'support': 15402}
micro
{'f1-score': 0.8943811019321363,
 'precision': 0.9429887705154045,
 'recall': 0.85053889105311,
 'support': 15402}
weighted
{'f1-score': 0.872625817247425,
 'precision': 0.9103154855696425,
 'recall': 0.85053889105311,
 'support': 15402}
## test
PPCR: 0.874161
### report
macro
{'f1-score': 0.5153868786997444,
 'precision': 0.5542239018229216,
 'recall': 0.5003281794268066,
 'support': 3126}
micro
{'f1-score': 0.8973128598848369,
 'precision': 0.8973128598848369,
 'recall': 0.8973128598848369,
 'support': 3126}
weighted
{'f1-score': 0.8801294769979489,
 'precision': 0.8756945874590928,
 'recall': 0.8973128598848369,
 'support': 3126}
### report_full
macro
{'f1-score': 0.4544035961031645,
 'precision': 0.5542239018229216,
 'recall': 0.41101487983926416,
 'support': 3576}
micro
{'f1-score': 0.8370635631154879,
 'precision': 0.8973128598848369,
 'recall': 0.7843959731543624,
 'support': 3576}
weighted
{'f1-score': 0.8112946920488717,
 'precision': 0.8647483704154895,
 'recall': 0.7843959731543624,
 'support': 3576}
```

## javascript
### Summary
7 rules, avg.len. 5.1

| | |
|-|-|
|Min support|125|
|Max support|7106|
|Min confidence|0.9215116500854492|
|Max confidence|0.9959999918937683|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type = JSXIdentifier<br>⇒ y = "<br>Confidence: 0.983. Support: 434.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.974. Support: 434.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 172.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 654.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.947. Support: 199.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 125.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {COMMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 7106.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.142857142857143, "max_conf": 0.9959999918937683, "max_support": 7106, "min_conf": 0.9215116500854492, "min_support": 125, "num_rules": 7}}
```
</details>
