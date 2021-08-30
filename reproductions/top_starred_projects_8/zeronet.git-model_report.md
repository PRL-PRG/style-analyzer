# Model report for file:///tmp/top-repos-quality-repos-4w_qnvtl/zeronet.git HEAD 454c0b2e7e000fda7000cba49027541fbf327b96

### Dump

```json
{'created_at': '2021-08-29 16:50:03',
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
 'size': '19.8 kB',
 'tags': [],
 'uuid': '6ee678bd-05ae-4148-8e24-18fa70b0f379',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-4w_qnvtl/zeronet.git 454c0b2e7e000fda7000cba49027541fbf327b96

# javascript
36 rules, avg.len. 8.7
## train
PPCR: 0.914588
### report
macro
{'f1-score': 0.6257050944450483,
 'precision': 0.6345146770189085,
 'recall': 0.624070535654064,
 'support': 68552}
micro
{'f1-score': 0.9658215661103979,
 'precision': 0.9658215661103979,
 'recall': 0.9658215661103979,
 'support': 68552}
weighted
{'f1-score': 0.959897154602774,
 'precision': 0.9548576796664331,
 'recall': 0.9658215661103979,
 'support': 68552}
### report_full
macro
{'f1-score': 0.5085090705315739,
 'precision': 0.6345146770189085,
 'recall': 0.4565416056556272,
 'support': 74954}
micro
{'f1-score': 0.9227349379120038,
 'precision': 0.9658215661103979,
 'recall': 0.8833284414440857,
 'support': 74954}
weighted
{'f1-score': 0.8958989160515693,
 'precision': 0.9159614520181325,
 'recall': 0.8833284414440857,
 'support': 74954}
## test
PPCR: 0.890419
### report
macro
{'f1-score': 0.6012755673826361,
 'precision': 0.6030174246546635,
 'recall': 0.6195484663190747,
 'support': 11441}
micro
{'f1-score': 0.9658246656760773,
 'precision': 0.9658246656760773,
 'recall': 0.9658246656760773,
 'support': 11441}
weighted
{'f1-score': 0.9598078282591832,
 'precision': 0.956106399795637,
 'recall': 0.9658246656760773,
 'support': 11441}
### report_full
macro
{'f1-score': 0.4647626438061175,
 'precision': 0.6030174246546635,
 'recall': 0.4298386676665909,
 'support': 12849}
micro
{'f1-score': 0.909839440098806,
 'precision': 0.9658246656760773,
 'recall': 0.8599891042104444,
 'support': 12849}
weighted
{'f1-score': 0.8802829648647805,
 'precision': 0.933506679993721,
 'recall': 0.8599891042104444,
 'support': 12849}
```

## javascript
### Summary
19 rules, avg.len. 8.4

| | |
|-|-|
|Min support|105|
|Max support|11641|
|Min confidence|0.9369834661483765|
|Max confidence|0.998975396156311|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 11641.` |
| 2 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 341.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 116.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 7300.` |
| 5 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 1107.` |
| 6 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -3.reserved = ;<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 111.` |
| 7 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 9261.` |
| 8 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 3951.` |
| 9 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 2105.` |
| 10 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {;}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 242.` |
| 11 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 778.` |
| 12 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ;}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = "<br>Confidence: 0.937. Support: 484.` |
| 13 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 488.` |
| 14 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 4<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = "<br>Confidence: 0.976. Support: 105.` |
| 15 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.length ≥ 2<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.968. Support: 141.` |
| 16 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 18<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 238.` |
| 17 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 187.` |
| 18 | `  -1.diff_col ≥ 7<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles in {NAME} and not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {EXPRESSION, IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 165.` |
| 19 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 8686.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.421052631578947, "max_conf": 0.998975396156311, "max_support": 11641, "min_conf": 0.9369834661483765, "min_support": 105, "num_rules": 19}}
```
</details>
